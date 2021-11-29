const logic = {
  getNDiverged: (x0: number, y0: number, max_iter: number) => {
    let xn = 0;
    let yn = 0;
    for (let i = 1; i < max_iter; i++) {
      const x_next = xn * xn - yn * yn + x0;
      const y_next = 2 * xn * yn + y0;
      xn = x_next;
      yn = y_next;
      if (xn * xn + yn * yn > 4) return i;
    }
    return max_iter;
  },
  generateMandelbrotSet: (
    canvas_w: number,
    canvas_h: number,
    x_min: number,
    x_max: number,
    y_min: number,
    y_max: number,
    max_iter: number
  ) => {
    const data: number[] = [];
    for (let i = 0; i < canvas_h; i++) {
      const y = y_min + ((y_max - y_min) * i) / canvas_h;
      for (let j = 0; j < canvas_w; j++) {
        const x = x_min + ((x_max - x_min) * j) / canvas_w;
        const iter_index = logic.getNDiverged(x, y, max_iter);
        const v = (iter_index % 8) * 32;
        data.push(v);
        data.push(v);
        data.push(v);
        data.push(255);
      }
    }
    return data;
  }
};

const draw = (
  ctx: CanvasRenderingContext2D,
  canvas_w: number,
  canvas_h: number,
  data: Uint8Array | number[]
) => {
  const img = new ImageData(new Uint8ClampedArray(data), canvas_w, canvas_h);
  ctx.putImageData(img, 0, 0);
};

const X_MIN = -1.5;
const X_MAX = 0.5;
const Y_MIN = -1.0;
const Y_MAX = 1.0;
const MAX_ITER = 64;

export default () => {
  console.log('start loading wasm');
  const mandelbrot = import('../pkg/mandelbrot').catch(console.error);

  mandelbrot.then(pkg => {
    console.log('finished loading wasm');
    if (!pkg) return;
    const { draw_mandelbrot_set, generate_mandelbrot_set } = pkg;
    const renderBtn = document.querySelector<HTMLButtonElement>('#render')!;
    renderBtn.addEventListener('click', () => {
      let jsResult: number[] | null = null;
      {
        console.log('js only');
        const CANVAS_ID = 'canvas_js';
        const canvas = document.querySelector<HTMLCanvasElement>(
          `#${CANVAS_ID}`
        )!;
        const context = canvas.getContext('2d')!;
        const canvasWidth = canvas.width;
        const canvasHeight = canvas.height;

        const generateStartTime = Date.now();
        jsResult = logic.generateMandelbrotSet(
          canvasWidth,
          canvasHeight,
          X_MIN,
          X_MAX,
          Y_MIN,
          Y_MAX,
          MAX_ITER
        );
        const generateEndTime = Date.now();
        const elapsed = generateEndTime - generateStartTime;
        console.log(`\tgenerate:js\tgenerate_elapsed:${elapsed}[ms]`);

        const drawStartTime = Date.now();
        draw(context, canvasWidth, canvasHeight, jsResult);
        const drawEndTime = Date.now();

        console.log(
          `\tdraw: js\tdraw_elapsed: ${drawEndTime - drawStartTime} [ms]`
        );
      }
      {
        console.log('wasm only');
        draw_mandelbrot_set();
      }
      let wasmReslut: Uint8Array | null = null;
      {
        console.log('wasm+js');
        const CANVAS_ID = 'canvas_hybrid';
        const canvas = document.querySelector<HTMLCanvasElement>(
          `#${CANVAS_ID}`
        )!;
        const context = canvas.getContext('2d')!;
        const canvasWidth = canvas.width;
        const canvasHeight = canvas.height;

        const generateStartTime = Date.now();
        wasmReslut = generate_mandelbrot_set(
          canvasWidth,
          canvasHeight,
          X_MIN,
          X_MAX,
          Y_MIN,
          Y_MAX,
          MAX_ITER
        );
        const generateEndTime = Date.now();
        const drawStartTime = Date.now();
        draw(context, canvasWidth, canvasHeight, wasmReslut);
        const drawEndTime = Date.now();
        const elapsed = generateEndTime - generateStartTime;
        console.log(`\tgenerate:wasm\tgenerate_elapsed:${elapsed}[ms]`);
        console.log(
          `\tdraw: js\tdraw_elapsed: ${drawEndTime - drawStartTime} [ms]`
        );
      }
      {
        let isSame = true;
        for (let i = 0; i < wasmReslut.length; i++) {
          if (wasmReslut[i] !== jsResult[i]) {
            console.log(i, wasmReslut[i], jsResult[i]);
            isSame = false;
            break;
          }
        }
        console.log(`(wasmReslut === jsResult):${isSame}`);
      }
    });
    const N = 1000;
    {
      const CANVAS_ID = 'canvas_hybrid';
      const result: number[] = [];
      const canvas = document.querySelector<HTMLCanvasElement>(
        `#${CANVAS_ID}`
      )!;
      const canvasWidth = canvas.width;
      const canvasHeight = canvas.height;

      const generateStartTime = Date.now();
      for (let i = 0; i < N; i++) {
        const iterStartTime = Date.now();
        generate_mandelbrot_set(
          canvasWidth,
          canvasHeight,
          X_MIN,
          X_MAX,
          Y_MIN,
          Y_MAX,
          MAX_ITER
        );
        result.push(Date.now() - iterStartTime);
      }
      const generateEndTime = Date.now();
      const elapsed = generateEndTime - generateStartTime;
      console.log(`\tgenerate:wasm\tgenerate_elapsed:${elapsed}[ms]/${N}iter`);
      console.log(JSON.stringify(result));
    }
    {
      const CANVAS_ID = 'canvas_js';
      const result: number[] = [];
      const canvas = document.querySelector<HTMLCanvasElement>(
        `#${CANVAS_ID}`
      )!;
      const canvasWidth = canvas.width;
      const canvasHeight = canvas.height;

      const generateStartTime = Date.now();
      for (let i = 0; i < N; i++) {
        const iterStartTime = Date.now();
        logic.generateMandelbrotSet(
          canvasWidth,
          canvasHeight,
          X_MIN,
          X_MAX,
          Y_MIN,
          Y_MAX,
          MAX_ITER
        );
        result.push(Date.now() - iterStartTime);
      }
      const generateEndTime = Date.now();
      const elapsed = generateEndTime - generateStartTime;
      console.log(`\tgenerate:js\tgenerate_elapsed:${elapsed}[ms]/${N}iter`);
      console.log(JSON.stringify(result));
    }
  });
};

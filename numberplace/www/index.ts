const PROBLEM_SET = [
  [
    0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 0, 0, 1, 0, 9, 0, 0, 0, 0, 5, 0,
    0, 0, 2, 0, 0, 3, 1, 0, 0, 0, 0, 1, 4, 0, 6, 7, 8, 0, 0, 0, 7, 8, 6, 5, 2,
    9, 0, 3, 4, 5, 6, 7, 0, 9, 3, 0, 0, 1, 8, 9, 1, 4, 5, 6, 3, 7, 2, 0, 2, 4,
    0, 8, 7, 5, 0, 9
  ],
  [
    1, 0, 0, 0, 0, 8, 0, 4, 0, 0, 0, 8, 0, 0, 9, 0, 6, 0, 0, 4, 5, 0, 0, 2, 8,
    0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 7, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 7, 0, 0, 7, 5, 0, 0, 6, 2, 0, 0, 5, 0, 6, 0, 0, 7, 0, 0, 0, 2, 0,
    3, 0, 0, 0, 0, 9
  ],
  [
    9, 0, 0, 7, 6, 0, 0, 5, 0, 0, 0, 6, 3, 0, 0, 0, 0, 0, 0, 1, 0, 9, 0, 0, 0,
    0, 4, 0, 2, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0,
    0, 0, 2, 0, 5, 0, 0, 0, 0, 8, 0, 7, 0, 0, 0, 0, 0, 0, 6, 3, 0, 0, 0, 4, 0,
    0, 3, 1, 0, 0, 8
  ],
  [
    0, 8, 0, 0, 0, 6, 5, 0, 0, 0, 0, 9, 0, 0, 7, 0, 0, 3, 5, 0, 0, 3, 0, 2, 0,
    6, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 5, 0, 0, 7, 0, 8, 0, 3, 0, 0, 4, 6, 0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 3,
    4, 0, 0, 0, 7, 0
  ]
];

const N = 81;
const isValid = (result: number[], p: number, v: number) => {
  const y = Math.floor(p / 9);
  const x = p % 9;
  for (let i = 0; i < 9; i++) {
    if (result[9 * i + x] === v || result[9 * y + i] === v) return false;
  }
  const block_y = Math.floor(y / 3);
  const block_x = Math.floor(x / 3);
  for (let i = 0; i < 3; i++) {
    for (let j = 0; j < 3; j++) {
      if (result[9 * (3 * block_y + i) + (3 * block_x + j)] === v) return false;
    }
  }
  return true;
};

const solveJs = (problem: number[]) => {
  const result: number[] = new Array(N).fill(0);

  const stack: [boolean, number, number][] = [];
  for (let i = 0; i < N; i++) {
    if (problem[i] > 0) {
      result[i] = problem[i];
    } else if (stack.length === 0) {
      stack.push([false, i, 1]);
    }
  }

  let isFailing = false;
  while (stack.length > 0) {
    const t = stack.pop()!;
    const isBack = t[0];
    const p = t[1];
    const v = t[2];
    if (isBack && isFailing) {
      result[p] = 0;
      if (v < 9) stack.push([false, p, v + 1]);
      continue;
    }
    if (!isValid(result, p, v)) {
      if (v < 9) {
        stack.push([false, p, v + 1]);
      } else {
        isFailing = true;
      }
      continue;
    }
    isFailing = false;
    result[p] = v;
    stack.push([true, p, v]);
    let isUpdated = false;
    for (let i = p + 1; i < N; i++) {
      if (result[i] === 0) {
        stack.push([false, i, 1]);
        isUpdated = true;
        break;
      }
    }
    if (!isUpdated) break;
  }
  return result;
};

export default () => {
  const wasm = import('../pkg/numberplace').catch(console.error);
  wasm.then(pkg => {
    if (!pkg) return;

    PROBLEM_SET.forEach(problem => {
      console.log('problem = ', problem);

      let ResultWasm;
      let ResultJs;
      {
        const _problem = new Uint8Array(problem);
        const solveStartTime = Date.now();
        ResultWasm = pkg.solve(_problem);
        const solveEndTime = Date.now();
        console.log(
          `solve:wasm\tsolve_elapsed:${solveEndTime - solveStartTime}[ms]`
        );
      }
      {
        const solveStartTime = Date.now();
        ResultJs = solveJs(problem);
        const solveEndTime = Date.now();
        console.log(
          `solve:js\tsolve_elapsed:${solveEndTime - solveStartTime}[ms]`
        );
      }

      for (let i = 0; i < 9; i++) {
        for (let j = 0; j < 9; j++) {
          if (ResultWasm[9 * i + j] !== ResultJs[9 * i + j]) {
            console.log(ResultWasm.slice(9 * i, 9 * i + 9));
            console.log(ResultJs.slice(9 * i, 9 * i + 9));
            break;
          }
        }
      }
    });
  });
};

const addArray = (n: number, x: number) => {
  const a = [...Array(n)].fill(0);
  [...Array(x)].forEach(() => {
    a.forEach((_, i) => {
      a[i] += 1;
    });
  });
  return a.reduce((a, c) => a + c, 0);
};

console.time();
console.log(addArray(parseInt(process.argv[2]), parseInt(process.argv[3])));
console.timeEnd();

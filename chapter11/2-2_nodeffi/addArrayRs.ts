import ffi from 'ffi-napi';
import ref from 'ref-napi';

const { uint } = ref.types;

const addArray = ffi.Library('addarray/target/release/addarray.dll', {
  add_array: [uint, [uint, uint]]
});

console.time();
console.log(
  addArray.add_array(parseInt(process.argv[2]), parseInt(process.argv[3]))
);
console.timeEnd();

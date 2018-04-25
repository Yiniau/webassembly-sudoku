const _Module_ = Symbol('_Module_');

window[_Module_] = {};

// const memory = new WebAssembly.Memory({initial: 20});

const imports = {
  env: {
    // memory: memory,
    log_true: () => console.log('check - result: true'),
    log_false: () => console.log('check - result: false'),
    log_num(num) { console.log(num); },
    log_ptr(ptr) { console.log(ptr); },
    random_int(s, e) {
      return Math.random() * (e - s) + s;
    }
  }
};

function fetchAndInstantiate(url, importObject) {
  return fetch(url).then(response =>
    response.arrayBuffer()
  )
    .then(bytes => WebAssembly.instantiate(bytes, importObject))
}

(async () => {
  try {
    // const { instance, module } = await fetchAndInstantiate("./sudoku/target/release/wasm32-unknown-unknown/release/sudoku.wasm", imports);
    const { instance, module } = await fetchAndInstantiate("./src/wasm/sudoku.wasm", imports);
    // const { instance, module } = await fetchAndInstantiate("./src/wasm/sudoku.wasm");
    const _mod_ = window[_Module_];
    _mod_.module = module;
    _mod_.instance = instance;
    _mod_.exports = instance.exports;
    // _mod_.memory = memory;
    _mod_.memory = instance.exports.memory;

    _mod_.alloc = instance.exports.alloc;
    _mod_.dealloc = instance.exports.dealloc;

    _mod_.log_matrix = (arr) => {
      let result = '';
      for (let y = 0; y < 9; y++) {
        result += `${arr[(y * 9)]} ${arr[1 + (y*9)]} ${arr[2 + (y*9)]} ${arr[3 + (y*9)]} ${arr[4 + (y*9)]} ${arr[5 + (y*9)]} ${arr[6 + (y*9)]} ${arr[7 + (y*9)]} ${arr[8 + (y*9)]} \n`;
      }
      console.log(result);
    };


    // get origin memory start pointer
    _mod_.new = () => {
      return instance.exports.new();
    };

    // check sudoku matrix is allowed
    _mod_.check = (ptr) => {
      let res = instance.exports.check(ptr);
      console.log('check - result: ', Boolean(res));
    };

    // init sudoku matrix
    _mod_.init = instance.exports.init;

    // get data
    _mod_.get_data = () => {
      let p = instance.exports.get_data;
      return new Uint8Array(
        _mod_.memory.buffer,
        p,
        81
      );
    };


    // create instance
    let originPtr = _mod_.new();
    _mod_.check = _mod_.check.bind(null, originPtr);
    console.log('memory address now: ', originPtr);

    // matrix check
    _mod_.check();

    let _p = _mod_.instance.exports.get_data();
    let _data = new Uint8Array(_mod_.memory.buffer, _p, 81);
    console.log('matrix data: ', _data);

    // init matrix
    let p = _mod_.init(originPtr);
    _mod_.data = new Uint8Array(_mod_.memory.buffer, p, 81);
    _mod_.log_matrix(_data);

    _mod_.check();

  } catch (e) {
    console.error(e);
    return;
  }

  console.log(window[Object.getOwnPropertySymbols(window)[0]]);
})();
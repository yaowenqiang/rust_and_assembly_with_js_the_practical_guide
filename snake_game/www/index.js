	// console.log("Hello World!");
	async function init() {
	const importObject = {
	  console: {
  			log: () => {
  			  console.log("just logging something!");
  			},
  			error: () => {
  			  console.log("I am just error!");
  			},
			}
	}
	  const response = await fetch('sum.wasm');
		const buffer = await response.arrayBuffer();
		const wasm = await WebAssembly.instantiate(buffer, importObject);
		const sumFunction = wasm.instance.exports.sum;
		const result = sumFunction(10,50);
		console.log(result);
	}

	init().catch(err => console.error("WASM 加载失败:", err));

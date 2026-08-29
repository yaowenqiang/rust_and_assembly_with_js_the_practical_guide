	// console.log("Hello World!");
	async function init() {
	  const response = await fetch('sum.wasm');
		const buffer = await response.arrayBuffer();
		const wasm = await WebAssembly.instantiate(buffer, {
			console: {
				log: () => {},
				error: () => {},
			},
			js: { mem: new WebAssembly.Memory({ initial: 1 }) },
		});
		const sumFunction = wasm.instance.exports.sum;
		const result = sumFunction(10,50);
		console.log(result);
	}

	init().catch(err => console.error("WASM 加载失败:", err));

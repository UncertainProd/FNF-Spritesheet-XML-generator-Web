import App from './App.svelte';
import wasm from '../../rust-img-lib/Cargo.toml';
import './styles.css';

const init = async () => {
	const helloWorldWasm = await wasm();
	helloWorldWasm.init_panic_hook();
	const app = new App({
		target: document.body,
		props: {
			wasm: helloWorldWasm
		}
	});
	return app;
};

export default init();
// export default app;

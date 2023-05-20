export * from "../rust-img-lib/pkg/rust_img_lib";

type Exports = typeof import("../rust-img-lib/pkg/rust_img_lib");
declare const init: () => Promise<Exports>;
export default init;
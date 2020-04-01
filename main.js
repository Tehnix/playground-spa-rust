import init, { run_app } from "./pkg/yew_app.js";

async function main() {
  await init("/pkg/yew_app_bg.wasm");
  run_app();
}

main();

import Context from "https://deno.land/std@0.127.0/wasi/snapshot_preview1.ts"

const context = new Context({
  args: Deno.args,
  env:  Deno.env.toObject(),
})

const binary = await Deno.readFile("/usr/local/bin/run.wasm")
const wasm   = await WebAssembly.compile(binary)
const built  = await WebAssembly.instantiate(wasm, {wasi_snapshot_preview1: context.exports})

context.start(built)

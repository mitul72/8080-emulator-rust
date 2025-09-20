compile to wasm:
wasm-pack build --target bundler --out-dir i8080-web/public/wasm -- --features wasm

remove wasm .gitignore:
Remove-Item -Path "i8080-web/public/wasm/.gitignore" -ErrorAction SilentlyContinue

complete wasm compile command:
wasm-pack build --target bundler --out-dir i8080-web/public/wasm -- --features wasm && Remove-Item -Path "i8080-web/public/wasm/.gitignore" -ErrorAction SilentlyContinue

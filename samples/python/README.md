```
wget https://github.com/ethanhs/python-wasm/suites/7175208612/artifacts/286125288
unzip wasi-main.zip
cd wasi-main
echo "print('hello world')" > test.py
runtime ./test.py
```
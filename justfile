format-haskell:
    ormolu --mode inplace $(find haskell -name '*.hs')

format-python:
    uv run ruff format pyaoc

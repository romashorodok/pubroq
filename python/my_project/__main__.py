from ._lowlevel import hello, RustStruct


def main() -> None:
    rust_struct = RustStruct(data="some data", vector=[255, 255, 255])
    rust_struct.extend_vector([1, 1, 1, 1])
    rust_struct.printer()
    print(hello())


if __name__ == "__main__":
    main()

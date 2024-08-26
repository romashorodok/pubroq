import asyncio
import signal
import sys

from ._lowlevel import hello, RustStruct
from watchfiles import run_process


async def coro():
    rust_struct = RustStruct(data="some data", vector=[255, 255, 255])
    rust_struct.extend_vector([1, 1, 1, 1])
    rust_struct.printer()
    print(hello())


def main_process():
    async def main_loop():
        task = asyncio.create_task(coro())
        try:
            await task
        except asyncio.CancelledError:
            print("Task was canceled")

    def signal_handler(sig, frame):
        print("Signal received, cancelling task")
        task.cancel()

    loop = asyncio.get_event_loop()
    task = loop.create_task(main_loop())
    loop.add_signal_handler(signal.SIGINT, signal_handler, signal.SIGINT, None)
    loop.add_signal_handler(signal.SIGTERM, signal_handler, signal.SIGTERM, None)

    try:
        loop.run_until_complete(task)
    except KeyboardInterrupt:
        print("Interrupted by user")
        task.cancel()
        loop.run_until_complete(task)
    except asyncio.CancelledError:
        print("Loop was cancelled")


def main() -> None:
    run_process("./python/my_project", target=main_process)


if __name__ == "__main__":
    main()

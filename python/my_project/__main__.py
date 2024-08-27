import asyncio
import signal
import json

from my_project import server
from watchfiles import run_process
from ._lowlevel import hello, RustStruct, human_says_hi, greater_than_2, CPythonAgent


async def coro():
    try:
        await server.start_server()
    except KeyboardInterrupt:
        pass
    # rust_struct = RustStruct(data="some data", vector=[255, 255, 255])
    # rust_struct.extend_vector([1, 1, 1, 1])
    # rust_struct.printer()
    # print(hello())
    # human = json.dumps({"name": "Someone", "age": 1})
    # human_says_hi(human)
    #
    # try:
    #     greater_than_2(1)
    #     raise RuntimeError("Must throw exception")
    # except ValueError:
    #     pass
    #
    # agent = CPythonAgent()
    # ufrag = agent.ufrag()
    # print(agent, ufrag)


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

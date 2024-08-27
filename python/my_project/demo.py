#
# demo application for http3_server.py
#

import datetime
import os
from urllib.parse import urlencode

from starlette.applications import Starlette
from starlette.responses import PlainTextResponse, Response
from starlette.routing import Mount, Route, WebSocketRoute
from starlette.staticfiles import StaticFiles
from starlette.templating import Jinja2Templates
from starlette.types import Receive, Scope, Send
from starlette.websockets import WebSocketDisconnect


async def wt(scope: Scope, receive: Receive, send: Send) -> None:
    """
    WebTransport echo endpoint.
    """
    # accept connection
    message = await receive()
    assert message["type"] == "webtransport.connect"
    await send({"type": "webtransport.accept"})

    # echo back received data
    while True:
        message = await receive()
        if message["type"] == "webtransport.datagram.receive":
            await send(
                {
                    "data": message["data"],
                    "type": "webtransport.datagram.send",
                }
            )
        elif message["type"] == "webtransport.stream.receive":
            await send(
                {
                    "data": message["data"],
                    "stream": message["stream"],
                    "type": "webtransport.stream.send",
                }
            )


async def app(scope: Scope, receive: Receive, send: Send) -> None:
    await wt(scope, receive, send)

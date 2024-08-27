import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

import { WasmAgent } from "../pkg";
import init from "../pkg";

function App() {
  const [count, setCount] = useState(0)

  useEffect(() => {
    init().then(() => {
      const test = new WasmAgent()
      console.log(test.ufrag())
      console.log(test)
    })
  }, [])

  useEffect(() => {
    const transport = new WebTransport("https://localhost:4433/counter", {
      requireUnreliable: true,
    });
    let intervalId: number;
    async function start() {
      await transport.ready;

      let stream = await transport.createBidirectionalStream();
      let reader = stream.readable.getReader();
      let writer = stream.writable.getWriter();

      await writer.write(new Uint8Array([65, 66, 67]));
      intervalId = setInterval(async () => {
        let received = await reader.read();
        console.log('received', received);
        await writer.write(new Uint8Array([65, 66, 67]));
      }, 1000)
    }
    start()

    return () => {
      transport.close()
      clearInterval(intervalId)
    }
  }, [])



  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App

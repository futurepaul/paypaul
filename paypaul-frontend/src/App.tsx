import { Component, createResource, onMount } from 'solid-js';

import { QRCodeSVG } from "solid-qr-code";
import { useCopy } from './useCopy';

async function fetchPaymentInfo() {
  let res = await fetch("http://localhost:3001/address");
  let json = res.json();
  console.log(json);
  return json;
}

const App: Component = () => {
  const [paymentInfo, { refetch }] = createResource("heyo", fetchPaymentInfo);

  const [copy, copied] = useCopy();

  return (
    <main class="min-h-[100vh] bg-gray-500 flex flex-col items-center justify-center">
      <div class="w-[50vw] max-h-[80vh] bg-white rounded md:rounded-xl shadow md:shadow-xl flex flex-col items-center">
        <QRCodeSVG value={paymentInfo()?.address} class="w-full h-full px-[5vw] pt-[5vw]" />
        <div class="w-full px-[5vw] pt-[3vw] pb-[2vw] flex justify-center items-center gap-2">
          <pre class="text-relative overflow-hidden text-ellipsis whitespace-nowrap">{paymentInfo()?.address}</pre>
          <button type="button" class="bg-gray-200 text-gray-800 p-2 rounded" onClick={() => copy(paymentInfo()?.address)}>{copied() ? "Copied" : "Copy"}</button>
        </div>
        <div class="pb-[2vw]">
          <button type="button" class="bg-gray-200 text-gray-800 p-2 rounded" onClick={() => refetch()}>Refresh</button>
        </div>
      </div>

    </main>
  );
};

export default App;

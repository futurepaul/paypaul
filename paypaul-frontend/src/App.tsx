import { Component, createResource, onMount, splitProps } from 'solid-js';

import { QRCodeSVG } from "solid-qr-code";
import { useCopy } from './useCopy';

async function fetchPaymentInfo() {
  let res = await fetch("http://localhost:3001/address");
  let json = res.json();
  console.log(json);
  return json;
}

const CopyRow: Component<{ text: string }> = (props) => {
  const [local] = splitProps(props, ["text"]);
  const [copy, copied] = useCopy();
  return (
    <div class="w-full flex justify-between items-center gap-2">
      <pre class="text-relative overflow-hidden text-ellipsis whitespace-nowrap">{local.text}</pre>
      <button type="button" class="bg-gray-200 text-gray-800 p-2 rounded" onClick={() => copy(local.text)}>{copied() ? "Copied" : "Copy"}</button>
    </div>
  )
}

const App: Component = () => {
  const [paymentInfo, { refetch }] = createResource("heyo", fetchPaymentInfo);


  return (
    <main class="min-h-[100vh] bg-gray-500 flex flex-col items-center justify-center">
      <div class="w-[50vw] max-h-[80vh] bg-white rounded md:rounded-xl shadow md:shadow-xl flex flex-col items-center">
        <QRCodeSVG value={paymentInfo()?.bip21} class="w-full h-full px-[5vw] pt-[5vh]" />
        <div class="flex flex-col max-w-[40vw] gap-2 pt-[5vh] pb-[5vh]">
          <CopyRow text={paymentInfo()?.bip21} />
          <CopyRow text={paymentInfo()?.address} />
          <CopyRow text={paymentInfo()?.bolt11} />
          <button type="button" class="bg-gray-200 text-gray-800 p-2 rounded" onClick={() => refetch()}>Refresh</button>
        </div>
      </div>

    </main>
  );
};

export default App;

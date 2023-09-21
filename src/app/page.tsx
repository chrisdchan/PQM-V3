'use client'
import { useState } from "react";
import SelectFiles from "./lib/selectFiles"
import SelectFolder from "./lib/selectFolder"

export default function Home() {

  const [graphResponse, setGraphResponse] = useState(null);

  return (
    <main className="flex min-h-screen flex-row items-center justify-center 
      bg-[#707070]">
      <SelectFiles />
      <SelectFolder />
    </main>
  )
}

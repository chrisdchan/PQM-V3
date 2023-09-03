import SelectFiles from "./lib/selectFiles"
import SelectFolder from "./lib/selectFolder"

export default function Home() {
  return (
    <main className="flex min-h-screen flex-row items-center justify-center 
      bg-[#707070]">
      <SelectFiles />
      <SelectFolder />
    </main>
  )
}

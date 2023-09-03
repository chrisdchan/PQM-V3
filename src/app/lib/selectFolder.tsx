'use client'

import { invoke } from "@tauri-apps/api"


export default function SelectFolder() {

    const handleSelectFolder = () => {
        invoke('select_folder')
        .then(console.log)
    }

    return (
        <div>
            <button className="border-2 rounded-md p-2 m-3 text-white hover:bg-[#808080]"
            onClick={handleSelectFolder}>Select Folder</button>
        </div>
    )

}
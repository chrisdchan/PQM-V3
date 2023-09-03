'use client'

import {invoke} from '@tauri-apps/api/tauri'

export default function SelectFiles() {

    const handleSelectFiles = () => {
        invoke('select_files')
        .then(console.log)
    }

    return (
        <div>
            <button className = "border-2 rounded-md p-2 m-3 text-white hover:bg-[#808080]"
            onClick={handleSelectFiles}>Select Files</button>
        </div>
    )

}
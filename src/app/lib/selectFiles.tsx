'use client'

import {invoke} from '@tauri-apps/api/tauri'
import { useRouter } from 'next/navigation'


export default function SelectFiles() {

    const router = useRouter();

    const handleSelectFiles = async () => {

        let graphResponse: JSON = await invoke('select_files')
        console.log(graphResponse);
        
        router.push('/graph/1234');
    }

    return (
        <div>
            <button className = "border-2 rounded-md p-2 m-3 text-white hover:bg-[#808080]"
            onClick={handleSelectFiles}>Select Files</button>
        </div>
    )

}
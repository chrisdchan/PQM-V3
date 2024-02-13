'use client'

import {invoke} from '@tauri-apps/api/tauri'
import {useRouter} from 'next/navigation'
import {GraphDisplay} from "@/app/types/api";


export default function SelectFiles() {

    const router = useRouter();

    const handleSelectFiles = async () => {

        const graphDisplay: GraphDisplay = await invoke<GraphDisplay>('select_files')
        console.log(graphDisplay);
        const routeName = `/graph/${graphDisplay.id}`
        router.push(routeName);
    }

    return (
        <div>
            <button className="border-2 rounded-md p-2 m-3 text-white hover:bg-[#808080]"
                    onClick={handleSelectFiles}>Select Files
            </button>
        </div>
    )

}
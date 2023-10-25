'use client'

import Graph from "@/app/lib/graph"
import { invoke } from "@tauri-apps/api"
import { useRouter } from "next/navigation"
import { useEffect, useState } from "react"
import { GraphDisplay } from "../../types/api"

const GraphPage = () => {

    const router = useRouter();
    let [currentGraph, setCurrentGraph] = useState<GraphDisplay|null>(null)

    const getCurrentGraph = async () => {
        try {
            const graphDisplay: GraphDisplay = await invoke('get_graph', {
                graphId: "graph_id"
            });
            
            setCurrentGraph(graphDisplay)
        } catch(err) {
            setCurrentGraph(null)
            console.log(err);
        }
    }

    useEffect(() => {
        getCurrentGraph()
    }, [])

    return (
        <div>
            <div className="w-32 h-32">
                <Graph graph={currentGraph} />
            </div>
            <button onClick={() => router.back()}>back</button>
        </div>
    )
}

export default GraphPage;

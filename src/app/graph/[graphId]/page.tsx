'use client'

import Graph from "@/app/lib/graph"
import { GraphDisplayProperties } from "@/app/types/graphDisplayProperties"
import { GraphResponse } from "@/app/types/response/graphResponse"
import { invoke } from "@tauri-apps/api"
import { useRouter } from "next/navigation"
import { useEffect, useState } from "react"

const GraphPage = () => {

    const router = useRouter();
    let [currentGraph, setCurrentGraph] = useState<GraphResponse|null>(null)

    const getCurrentGraph = async () => {
        try {
            console.log("Calling get_graph");
            
            const graphResponse: GraphResponse = await invoke('get_graph', {
                graphId: "graph_id"
            })
            console.log(graphResponse);
            setCurrentGraph(graphResponse)
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
            <Graph graph={currentGraph}/>
            <button onClick={() => router.back()}>back</button>
        </div>
    )
}

export default GraphPage;

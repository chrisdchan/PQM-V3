'use client'

import Graph from "@/app/lib/graph"
import { invoke } from "@tauri-apps/api"
import { useRouter } from "next/navigation"
import { useEffect, useState } from "react"
import { GraphDisplay } from "../../types/api"
import Table from "@/app/lib/graphTable";
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
            <div>
                <Graph graph={currentGraph} />
            </div>
            <div>
                <Table graphId={"graph_id"}/>
            </div>
            <button onClick={() => router.back()}>back</button>
        </div>
    )
}
export default GraphPage;

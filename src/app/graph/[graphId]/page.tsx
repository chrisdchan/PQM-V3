'use client'

import Graph from "@/app/lib/graph"
import {invoke} from "@tauri-apps/api"
import {useRouter} from "next/navigation"
import {useEffect, useState} from "react"
import {GraphDisplay} from "../../types/api"
import Table from "@/app/lib/graphTable";

const GraphPage = ({params}: { params: { graphId: string } }) => {

    const router = useRouter();
    let [currentGraph, setCurrentGraph] = useState<GraphDisplay | null>(null)
    const getCurrentGraph = async () => {
        try {
            const graphDisplay: GraphDisplay = await invoke('get_graph', {
                graphId: params.graphId
            });

            setCurrentGraph(graphDisplay)
        } catch (err) {
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
                {currentGraph !== null && <Graph graph={currentGraph}/>}
            </div>
            <div>
                <Table graphId={params.graphId}/>
            </div>
            <button onClick={() => router.back()}>back</button>
        </div>
    )
}
export default GraphPage;

import {GraphDisplay, GraphTableDisplay, StructureTableDisplay} from "@/app/types/api";
import {invoke} from "@tauri-apps/api";
import React, {useEffect, useState} from "react";

interface GraphTableProps {
    graphId: string
}

const GraphTable: React.FC<GraphTableProps> = ({graphId}) => {
    const [graphTableDisplay, setGraphTableDisplay] =
        useState<GraphTableDisplay|null>(null)

    const getCurrentTable = async () => {
        try {
            const graphTableDisplay: GraphTableDisplay = await invoke('get_graph_table', {
                graphId: graphId
            });
            console.log(graphTableDisplay)

            setGraphTableDisplay(graphTableDisplay);
        } catch(err) {
            setGraphTableDisplay(null)
            console.log(err);
        }
    }

    useEffect(() => {
        getCurrentTable()
    }, []);

    return (
        <div>
            <table>
                <tr>
                    <td>structure name</td>
                    <td>100</td>
                </tr>
                <tr>
                    <td>Structure</td>
                    <td>20</td>
                </tr>
            </table>
        </div>
    )
}

export default GraphTable;
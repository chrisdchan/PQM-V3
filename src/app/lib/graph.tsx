import { useCallback, useEffect, useRef, useState } from "react";
import { Graphics, Stage } from "@inlet/react-pixi";
import * as PIXI from 'pixi.js'
import { GraphDisplay } from "../types/api";
import { drawGraph } from "../util/drawUtil";

interface GraphProps {
    graph: GraphDisplay | null;
}

const Graph: React.FC<GraphProps> = ({graph}) => {
    const graphicsRef = useRef<PIXI.Graphics | null>(null);
    const [windowDimensions, setWindowDimensions] = useState({
        width: window.innerWidth,
        height: window.innerHeight,
    });


    useEffect(() => {
        const handleResize = () => {
            setWindowDimensions({
                width: window.innerWidth,
                height: window.innerHeight
            })
        };
        
        window.addEventListener('resize', handleResize);

        return () => {
            window.removeEventListener('resize', handleResize);
        }
    }, []);

    useEffect(() => {
        if(graphicsRef.current) {
            drawGraph(graphicsRef.current, graph);
        }
    }, [graph, windowDimensions])

    // Set the desired percentage of the window size
    const stageWidthPercentage = 0.8; // 80%
    const stageHeightPercentage = 0.8; // 80%

    const stageWidth = windowDimensions.width * stageWidthPercentage;
    const stageHeight = windowDimensions.height * stageHeightPercentage;

    return(
            <Stage width={stageWidth} height={stageHeight}>
                <Graphics ref={graphicsRef}/>
            </Stage>
    );
}

export default Graph;
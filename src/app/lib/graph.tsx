import { useCallback, useEffect, useRef, useState } from "react";
import { Graphics, Stage } from "@inlet/react-pixi";
import * as PIXI from 'pixi.js'
import { GraphDisplay } from "../types/api";
import { drawGraph } from "../util/drawGraphUtil";
import { drawFigure } from "../util/drawFigureUtil";

interface GraphProps {
    graph: GraphDisplay | null;
}

export type WindowDimensions =  {
    height: number,
    width: number
}

const Graph: React.FC<GraphProps> = ({graph}) => {
    const foregroundGraphicsRef = useRef<PIXI.Graphics | null>(null);
    const backgrounGrapcisRef = useRef<PIXI.Graphics | null>(null);

    const [windowDimensions, setWindowDimensions] = useState<WindowDimensions>({
        width: window.innerWidth * 0.8,
        height: window.innerHeight * 0.8,
    });

    useEffect(() => {
        const handleResize = () => {
            setWindowDimensions({
                width: window.innerWidth * 0.8,
                height: window.innerHeight * 0.8
            })
        };

        window.addEventListener('resize', handleResize);

        return () => {
            window.removeEventListener('resize', handleResize);
        }
    }, []);

    useEffect(() => {
        if(backgrounGrapcisRef.current && foregroundGraphicsRef.current) {
            if(graph){
                drawFigure(
                    backgrounGrapcisRef.current,
                    foregroundGraphicsRef.current,
                    windowDimensions, 
                    graph)
            }
        }
    }, [graph, windowDimensions])


    const backgroundColorStr = graph?.graphDisplayStyle.outerColor as string
    const backgroundColor = parseInt(backgroundColorStr, 16);

    return(
        <Stage 
            width={windowDimensions.width} 
            height={windowDimensions.height}
            options={{backgroundColor: 0xAAFFFF }}
            >
            <Graphics ref={foregroundGraphicsRef}/>
            <Graphics ref={backgrounGrapcisRef}/>
        </Stage>
    );
}

export default Graph;
import { useCallback, useEffect, useState } from "react";
import { GraphContext } from "../types/graphContext";
import { Graphics, Stage } from "@inlet/react-pixi";
import * as PIXI from 'pixi.js'
import { GraphResponse } from "../types/response/graphResponse";

interface GraphProps {
    graph: GraphResponse | null;
}

const Graph: React.FC<GraphProps> = ({graph}) => {
    const [windowDimensions, setWindowDimensions] = useState({
        width: window.innerWidth,
        height: window.innerHeight,
    });

    const draw = (g: PIXI.Graphics) => {
    g.clear();
    g.beginFill(0xff0000);
    g.drawRect(0, 0, 100, 100);
    g.endFill();
    }

    useEffect(() => {
        draw
    }, [graph])

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

    // Set the desired percentage of the window size
    const stageWidthPercentage = 0.8; // 80%
    const stageHeightPercentage = 0.8; // 80%

    const stageWidth = windowDimensions.width * stageWidthPercentage;
    const stageHeight = windowDimensions.height * stageHeightPercentage;

    return(
            <Stage width={stageWidth} height={stageHeight}>
                <Graphics draw={draw}/>
            </Stage>
    );
}

export default Graph;
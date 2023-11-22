import * as Vditor from "vditor";
import "vditor/dist/index.css";
import {useEffect, useState} from "react";
import "./VditorEdit.scss"

export default function VditorEdit() {
    const [vd, setVd] = useState<Vditor>();
    useEffect(() => {
        const vditor = new Vditor("vditor", {
            height: '100%',
            minHeight: 200,
            resize: {
                enable: false
            },
            after: () => {
                vditor.setValue("`Vditor` 最小代码示例");
                setVd(vditor);
            }
        });
    }, []);

    return (
            <div id="vditor" className="vditor-div"/>
    ) ;
}
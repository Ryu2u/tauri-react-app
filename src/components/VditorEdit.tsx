import * as Vditor from "vditor";
import "vditor/dist/index.css";
import {useEffect, useState} from "react";
import "./VditorEdit.scss"

export default function VditorEdit({getVditor}) {

    useEffect(() => {
        const vditor = new Vditor("vditor", {
            height: '100%',
            minHeight: 200,
            resize: {
                enable: false
            },
            toolbar:
                [
                    "headings",
                    "bold",
                    "italic",
                    "strike",
                    "upload",
                    "inline-code",
                    "link",
                    "edit-mode",
                    'fullscreen'
                ]
            ,
            after: () => {
                vditor.setValue("`Vditor` 最小代码示例");

                getVditor(vditor);
            }
        });
    }, []);

    return (
        <div id="vditor" className="vditor-div"/>
    );
}
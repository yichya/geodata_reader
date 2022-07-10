'use strict';
'require uci';
'require view';

return view.extend({
    handleSaveApply: null,
    handleSave: null,
    handleReset: null,
    render: function() {
        var body = E([
            E('h2', _('GeoData Reader'))
        ]);
        var canvas = E('<div style="position: absolute; left: 50%; height: 750px;"><canvas id="the_canvas_id" style="margin-right: auto; margin-left: auto; display: block; position: absolute; left: 50%; transform: translate(-50%, 0%);"></canvas></div>')
        var script_wasm = E('script', {'src': '/geodata_reader.js'});
        body.appendChild(canvas);
        body.appendChild(script_wasm);
        return body;
    }
});

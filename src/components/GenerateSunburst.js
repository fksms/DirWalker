import * as d3 from 'd3'

function generateSunburst(data) { // returnの型は(SVGSVGElement | null)

    // SVG（viewBox）のサイズ
    const width = 500;
    const height = width;

    // 中心円の半径
    const radius = 65;

    // 画面に表示される深さ
    const visibleDepth = 5;

    // 円弧の角度[degree]が小さいものはパスから除外する
    const angleThreshold = 2.0;

    // アニメーションの遷移時間[msec]
    const transitionDuration = 750;


    // HierarchyNodeの作成
    //
    // sum: childrenの要素が0のもののサイズのみを足しこんでHierarchyNodeを作成
    // sort: サイズを昇順でソート
    const hierarchy = d3.hierarchy(data)
        .sum(d => d.children.length ? 0 : d.size)
        .sort((a, b) => b.value - a.value);


    // パーティションデータの作成
    //
    // 横の長さが2π、縦の長さが1のつららチャート（Icicle Chart）を作成する。
    // (x0, y0): 左上の座標
    // (x1, y1): 右下の座標
    const root = d3.partition().size([2 * Math.PI, hierarchy.height + 1])(hierarchy);
    // 各ノードに current プロパティを設定する
    root.each(d => d.current = {
        x0: d.x0,
        x1: d.x1,
        y0: d.y0,
        y1: d.y1
    });


    // カラースケールの作成
    //
    // scaleOrdinal: 配列の繰り返し設定を行う
    const rgbColorWheel = ["#FF0000", "#FF8000", "#FFFF00", "#80FF00", "#00FF00", "#00FF80", "#00FFFF", "#0080FF", "#0000FF", "#8000FF", "#FF00FF", "#FF0080"];
    const color = d3.scaleOrdinal().range(rgbColorWheel);


    // 円弧（arc）の生成
    //
    // 上記で設定したパーティションデータ（長方形）を円弧（バウムクーヘン形）に変換
    // （横の長さを2πと指定しているため、直接角度として変換できる）
    //
    // startAngle: 円弧の開始角度
    // endAngle: 円弧の終了角度
    // padAngle: 円弧間の間隔（パディング角度）
    // padRadius: 円弧間の間隔（パディング半径）
    // innerRadius: 円弧の内側の半径
    // outerRadius: 円弧の外側の半径
    const arcHeight = (width - radius * 2) / (visibleDepth * 2); // 円弧の外側の半径-円弧の内側の半径
    const arc = d3.arc()
        .startAngle(d => d.x0)
        .endAngle(d => d.x1)
        .padAngle(0.01)
        .padRadius(radius)
        .innerRadius(d => radius + (d.y0 - 1) * arcHeight)
        .outerRadius(d => radius + (d.y1 - 1) * arcHeight - 1);


    // svgタグの作成
    //
    // viewBox="x, y, width, height"
    // x: 左上のx座標
    // y: 左上のy座標
    // width: ビューポートの幅
    // height: ビューポートの高さ
    const svg = d3.create("svg")
        .attr("viewBox", [-width / 2, -height / 2, width, height])
        .attr("preserveAspectRatio", "xMidYMid meet")
        .style("width", "100%")
        .style("height", "100%");


    // 中心の円のパスを設定
    //
    // datum: 単一のエレメントを作成
    const parent = svg.append("circle")
        .datum(root)
        .attr("r", radius)
        .attr("fill", "none")
        .attr("pointer-events", "all")
        .on("click", (event, d) => clicked(event, d));


    // Arcを作成
    appendArc(0, true);


    // 初回のアニメーション（初回は不透明度を0に設定してから1になるようにフェードインさせる）
    svg.selectAll("path")
        .attr("fill-opacity", 0);
    svg.selectAll("path")
        .transition()
        .duration(transitionDuration)
        .ease(d3.easeLinear)
        .attr("fill-opacity", 1);


    return svg.node();



    // visibleDepthで指定した値より深いノードはfalseを返す
    //
    // node: オブジェクト
    // minDepth: 表示されるグラフの最も内側のDepth
    function arcVisible(node, minDepth) {
        return node.depth > minDepth && node.depth <= (visibleDepth + minDepth) && node.x1 > node.x0;
    }


    // Arcのパスを設定
    //
    // minDepth: 表示されるグラフの最も内側のDepth
    // isFirstCalled: 初めて呼ばれたか否か
    function appendArc(minDepth, isFirstCalled) {

        let coordinates = null;

        const squashedArcs = new Map();

        // --------------------ここからmain-arc用--------------------
        // 指定した要素を全て選択
        svg.selectAll("path.main-arc")
            // データ配列を作成
            .data(root.descendants().filter(d => {
                // visibleDepthより深い階層のものはパスから除外する
                if (!arcVisible(d, minDepth)) return false;

                // First Called
                if (isFirstCalled) { coordinates = d.current; }
                // Update
                else { coordinates = d.target; }

                // 円弧の角度[degree]が小さいものは除外する
                if ((coordinates.x1 - coordinates.x0) < (angleThreshold * Math.PI / 180)) {

                    // --------------------ここからsquashed-arc用--------------------
                    const parentName = d.parent.data.name;

                    // 除外された円弧のparentNameが、まだMapに格納されていない場合
                    if (!squashedArcs.has(parentName)) {
                        // 座標のコピーを作成（参照渡しさせないため）
                        const newCoordinates = {
                            x0: coordinates.x0,
                            x1: coordinates.x1,
                            y0: coordinates.y0,
                            y1: coordinates.y1
                        }
                        // 除外された円弧のparentNameをキーとして、座標を格納
                        squashedArcs.set(parentName, newCoordinates);
                    }
                    // 除外された円弧のparentNameが、既にMapに格納されている場合
                    else {
                        // 格納された座標を取得
                        let tmpCoordinates = squashedArcs.get(parentName);
                        tmpCoordinates.x1 = coordinates.x1;
                        // 座標をUpdate
                        squashedArcs.set(parentName, tmpCoordinates);
                    }
                    // --------------------ここまでsquashed-arc用--------------------

                    return false;
                }

                // 上記で除外されなかったもの
                return true;
            }))
            // データ配列とDOMを結合
            .join("path")
            // classを設定
            .classed("main-arc", true)
            // d属性（パス）を設定
            .attr("d", d => {
                // First Called
                if (isFirstCalled) { coordinates = d.current; }
                // Update
                else { coordinates = d.target; }

                return arc(coordinates);
            })
            // fill属性（塗りつぶし）を設定
            .attr("fill", d => {
                // 子ノードがあるかどうかをチェック
                if (d.children) {
                    // 子ノードがある場合

                    // 明度尺度を設定
                    let lightnessOrdinal = 1;
                    if (d.depth <= visibleDepth) { lightnessOrdinal = d.depth }
                    else { lightnessOrdinal = visibleDepth }

                    // 深さが1になるまで親ノードを辿る
                    while (d.depth > 1) { d = d.parent; }

                    // 階層が深くなるごとに明るくする
                    const newColor = d3.scaleLinear().domain([1, visibleDepth + 2]).range([color(d.data.name), "#FFFFFF"])

                    return newColor(lightnessOrdinal);
                } else {
                    // 子ノードが無い場合
                    return d3.color("#C6C6C6"); // グレーを返す
                }
            })
            // fill-opacity属性（塗りつぶしの透明度）を設定
            .attr("fill-opacity", d => arcVisible(d, minDepth) ? 1 : 0)
            // ポインターイベントの設定
            .attr("pointer-events", d => arcVisible(d, minDepth) ? "auto" : "none")
            // arcにカーソルを合わせた時
            .on("mouseenter", (event, d) => { /* doSomething */ })
            // arc上で右クリックした時
            .on("contextmenu", (event, d) => {
                //event.preventDefault();
                /* doSomething */
            });

        // 子ノードを持っている場合はクリック可能に
        svg.selectAll("path.main-arc")
            .filter(d => d.children)
            .style("cursor", "pointer")
            .on("click", (event, d) => clicked(event, d));

        // ラベルの設定
        svg.selectAll("path.main-arc")
            .append("title")
            .text(d => `${d.data.name}`);
        // --------------------ここまでmain-arc用--------------------

        // --------------------ここからsquashed-arc用--------------------
        // 除外された円弧を圧縮したものを表示
        squashedArcs.forEach((value, key) => {
            // 円弧の角度[degree]がangleThresholdより大きいもののみ表示
            if ((value.x1 - value.x0) > (angleThreshold * Math.PI / 180)) {
                svg.append("path")
                    // classを設定
                    .classed("squashed-arc", true)
                    // d属性（パス）を設定
                    .attr("d", d => arc(value))
                    // fill属性（塗りつぶし）を設定
                    .attr("fill", d => d3.color("#5F5F5F"))
                    // arcにカーソルを合わせた時
                    .on("mouseenter", (event, d) => { /* doSomething */ });
            }
        });

        // ラベルの設定
        svg.selectAll("path.squashed-arc")
            .append("title")
            .text("test");
        // --------------------ここまでsquashed-arc用--------------------
    }


    // クリック時の動作
    //
    // p: クリックされた円弧のデータ
    function clicked(event, p) {
        // 中心円をクリックした場合、"circle"タグのdatumにクリックされた円弧の親を設定、parentが存在しない場合はrootを設定
        parent.datum(p.parent || root);

        // クリックされた円弧の移動先を設定
        // "each"によって全てのノードについて設定を行う
        //
        // p: 移動前のノードデータ
        // d: 移動後のノードデータ
        root.each(d => d.target = {
            x0: Math.max(0, Math.min(1, (d.x0 - p.x0) / (p.x1 - p.x0))) * 2 * Math.PI,
            x1: Math.max(0, Math.min(1, (d.x1 - p.x0) / (p.x1 - p.x0))) * 2 * Math.PI,
            y0: Math.max(0, d.y0 - p.depth),
            y1: Math.max(0, d.y1 - p.depth)
        });

        // 移動先のノードの深さ
        const targetDepth = p.depth;

        // "path"の要素を全て削除
        svg.selectAll("path").remove();

        // Arcを作成
        appendArc(targetDepth, false);

        // 変更を反映させる
        svg.enter();

        // --------------------ここからsquashed-arc用--------------------
        svg.selectAll("path.squashed-arc")
            .attr("fill-opacity", 0);
        // --------------------ここまでsquashed-arc用--------------------

        // --------------------ここからmain-arc用--------------------
        // d.current: 移動前の円弧の座標
        // d.target: 移動後の円弧の座標
        svg.selectAll("path.main-arc")
            .transition()
            .duration(transitionDuration)
            .tween("scale", d => {
                const i = d3.interpolate(d.current, d.target);
                return t => d.current = i(t);
            })
            .attrTween("d", d => () => arc(d.current))
            // 終了後に実行
            .on("end", (event, d) => {
                // --------------------ここからsquashed-arc用--------------------
                svg.selectAll("path.squashed-arc")
                    .transition()
                    .duration(transitionDuration / 3)
                    .ease(d3.easeLinear)
                    .attr("fill-opacity", 1);
                // --------------------ここまでsquashed-arc用--------------------
            });
        // --------------------ここまでmain-arc用--------------------
    }

}

export { generateSunburst }
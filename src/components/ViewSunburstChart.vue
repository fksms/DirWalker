<script setup>

import { ref, watch } from "vue";

import * as d3 from "d3";

// 親から渡されたコンポーネントの参照を受け取る
const props = defineProps(["viewDirectoryFileList"]);


// DOM格納用
const svgDOM = ref();

// DOMへの参照
const svgDOMRef = ref();

// svgContainerの変更を監視して、描画を更新する
watch(svgDOM, (newValue) => {
    const container = svgDOMRef.value;
    if (container && newValue) {
        // 既存の描画をクリア
        container.innerHTML = '';
        // 新しいSVGSVGElementを追加
        container.appendChild(newValue);
    }
});


// --------------------パラメータ--------------------

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
const transitionDuration = 600;

// マウスホバーしてからListに反映されるまでの時間[msec]
const hoverTimeout = 400;

// マウスホバーした際の点滅の間隔[msec]
const blinkInterval = 600;

// ディレクトリのカラーコード（原色）
const directoryColorCodes = ["#FF0000", "#FF8000", "#FFFF00", "#80FF00", "#00FF00", "#00FF80", "#00FFFF", "#0080FF", "#0000FF", "#8000FF", "#FF00FF", "#FF0080"];

// ファイルのカラーコード（ライトグレー）
const fileColorCode = "#C6C6C6";

// squashされた部分のカラーコード（ダークグレー）
const squashedColorCode = "#777777";

// --------------------パラメータ--------------------

// --------------------グローバル--------------------

// Hierarchy data
let hierarchy = null;

// Partition data
let partition = null;

// SVG element data
let svgElement = null;

// --------------------グローバル--------------------


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


// Sunburstの作成
//
// returnの型は(SVGSVGElement | null)
function generateSunburst(data) {

    // HierarchyNodeの作成
    //
    // sum: childrenの要素が0のもののサイズのみを足しこんでHierarchyNodeを作成
    // sort: サイズを昇順でソート
    hierarchy = d3.hierarchy(data)
        .sum(d => d.children.length ? 0 : d.size)
        .sort((a, b) => b.value - a.value);

    // パーティションデータの作成
    //
    // 横の長さが2π、縦の長さが1のつららチャート（Icicle Chart）を作成する。
    // (x0, y0): 左上の座標
    // (x1, y1): 右下の座標
    partition = d3.partition().size([2 * Math.PI, hierarchy.height + 1])(hierarchy);

    // カラースケールの作成
    //
    // scaleOrdinal: 配列の繰り返し設定を行う
    const colorWheel = d3.scaleOrdinal().range(directoryColorCodes);

    // 各ノードにプロパティを追加する
    //
    // each: ノードを幅優先で呼び出す
    partition.each(d => {
        // currentプロパティの追加
        d.current = {
            x0: d.x0,
            x1: d.x1,
            y0: d.y0,
            y1: d.y1
        };

        // colorプロパティの追加
        //
        // 子ノードがある場合
        if (d.children) {
            // depthが0以下の場合
            if (d.depth <= 0) {
                d.color = "#FFFFFF00"; // 透明
            }
            // depthが1の場合
            else if (d.depth == 1) {
                d.color = rgb2Hex(colorWheel(d.data.name)); // 原色
            }
            // depthが1より大きくvisibleDepth以下の場合
            else if (d.depth > 1 && d.depth <= visibleDepth) {

                // 考え方
                //
                // depth=1: 「原色/白」のカラーをX等分して、2つ目
                // depth=2: 「depth=1/白」のカラーをX-1等分して、2つ目
                // depth=3: 「depth=2/白」カラーをX-2等分して、2つ目
                // ...
                const newColorArray = d3.scaleLinear().domain([0, visibleDepth - d.depth + 3]).range([d.parent.color, "#FFFFFF"]);
                d.color = rgb2Hex(newColorArray(1)); // 階層が深くなるごとに明るくなる
            }
            // depthがvisibleDepthより大きい場合
            else {
                d.color = d.parent.color; // 親のカラーと同じ
            }
        }
        // 子ノードが無い場合 
        else {
            d.color = fileColorCode; // ライトグレー
        }
    });

    // svgタグの作成
    //
    // viewBox="x, y, width, height"
    // x: 左上のx座標
    // y: 左上のy座標
    // width: ビューポートの幅
    // height: ビューポートの高さ
    svgElement = d3.create("svg")
        .attr("viewBox", [-width / 2, -height / 2, width, height])
        .attr("preserveAspectRatio", "xMidYMid meet")
        .style("width", "100%")
        .style("height", "100%");

    // 中心の円のパスを設定
    //
    // datum: 単一のエレメントを作成
    svgElement.append("circle")
        .datum(partition)
        .attr("r", radius)
        .attr("fill", "none")
        .attr("pointer-events", "all")
        .on("click", (event, d) => leftClicked(d.parent))
        // カーソルを合わせた時
        .on("mouseenter", (event, d) => { mouseEntered(event, d) })
        // カーソルを離した時
        .on("mouseleave", (event, d) => { mouseLeaved(event) });

    // Arcを描画
    drawArc(0, true);

    // FileSizeを描画
    drawFileSize(partition.value);

    // 初回のアニメーション（初回は不透明度を0に設定してから1になるようにフェードインさせる）
    svgElement.selectAll("path")
        .attr("fill-opacity", 0);
    svgElement.selectAll("path")
        .transition()
        .duration(transitionDuration)
        .ease(d3.easeLinear)
        .attr("fill-opacity", 1);

    // 初回のアニメーション（初回は不透明度を0に設定してから1になるようにフェードインさせる）
    svgElement.selectAll("text")
        .attr("fill-opacity", 0);
    svgElement.selectAll("text")
        .transition()
        .duration(transitionDuration)
        .ease(d3.easeLinear)
        .attr("fill-opacity", 1);

    // Listの更新
    updateList(partition);

    // DOMを格納
    svgDOM.value = svgElement.node();
}


// visibleDepthで指定した値より深いノードはfalseを返す
//
// node: オブジェクト
// lowerDepth: 表示されるグラフの最も内側のDepth
function arcVisible(node, lowerDepth) {
    return node.depth > lowerDepth && node.depth <= (visibleDepth + lowerDepth) && node.x1 > node.x0;
}


// TB/GB/MB/KBに変換
function toReadable(value) {
    return props.viewDirectoryFileList.toReadable(value);
}


// rgb形式からhex形式に変換
function rgb2Hex(rgb) {
    const hex = d3.color(rgb).formatHex();
    return hex.toUpperCase();
}


// FileSizeを描画
//
// fileSize: ファイルサイズを入力
function drawFileSize(fileSize) {
    svgElement.selectAll("text")
        .data(toReadable(fileSize))
        .join("text")
        .attr("text-anchor", "middle")
        .attr("fill", "#FFFFFF")
        .attr("x", 0)
        .attr("y", (d, i) => i * 35 - 5)
        .attr("font-size", "2em")
        .attr("pointer-events", "none")
        .text(d => d);
}


// Arcを描画
//
// lowerDepth: 表示されるグラフの最も内側のDepth
// isFirstCalled: 初めて呼ばれたか否か
function drawArc(lowerDepth, isFirstCalled) {

    // 座標格納用
    let coordinates = null;

    // Mapオブジェクト
    const squashedArcs = new Map();

    // --------------------ここからmain-arc用--------------------
    // 指定した要素を全て選択
    svgElement.selectAll("path.main-arc")
        // データ配列を作成
        .data(partition.descendants().filter(d => {
            // visibleDepthより深い階層のものはパスから除外する
            if (!arcVisible(d, lowerDepth)) return false;

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
                    // オブジェクトの作成
                    const squashedObject = {
                        parentNode: d.parent,
                        head: d.value, // squashされた部分で最もサイズの大きいノードのvalue
                        x0: coordinates.x0,
                        x1: coordinates.x1,
                        y0: coordinates.y0,
                        y1: coordinates.y1
                    }
                    // 除外された円弧のparentNameをキーとして、座標を格納
                    squashedArcs.set(parentName, squashedObject);
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
        .attr("fill", d => d.color)
        // fill-opacity属性（塗りつぶしの透明度）を設定
        .attr("fill-opacity", d => arcVisible(d, lowerDepth) ? 1 : 0)
        // ポインターイベントの設定
        .attr("pointer-events", d => arcVisible(d, lowerDepth) ? "auto" : "none")
        // カーソルを合わせた時
        .on("mouseenter", (event, d) => { mouseEntered(event, d) })
        // カーソルを離した時
        .on("mouseleave", (event, d) => { mouseLeaved(event) })
        // 右クリックした時
        .on("contextmenu", (event, d) => {
            //event.preventDefault();
            /* doSomething */
        });

    // 子ノードを持っている場合はクリック可能に
    svgElement.selectAll("path.main-arc")
        .filter(d => d.children)
        .style("cursor", "pointer")
        .on("click", (event, d) => leftClicked(d));
    // --------------------ここまでmain-arc用--------------------

    // --------------------ここからsquashed-arc用--------------------
    // 除外された円弧を圧縮したものを表示
    squashedArcs.forEach((value, key) => {
        // 円弧の角度[degree]が大きいもののみ表示
        if ((value.x1 - value.x0) > (angleThreshold * Math.PI / 180)) {

            // リスト生成時のオプションを指定
            const option = {
                threshold: value.head // リスト生成時の閾値
            }

            // path要素を生成
            svgElement.append("path")
                // データを作成
                .datum(value.parentNode)
                // classを設定
                .classed("squashed-arc", true)
                // d属性（パス）を設定
                .attr("d", d => {
                    coordinates.x0 = value.x0;
                    coordinates.x1 = value.x1;
                    coordinates.y0 = value.y0;
                    coordinates.y1 = value.y1;
                    return arc(coordinates);
                })
                // fill属性（塗りつぶし）を設定
                .attr("fill", squashedColorCode) // ダークグレー
                // カーソルを合わせた時
                .on("mouseenter", (event, d) => { mouseEntered(event, d) })
                // カーソルを離した時
                .on("mouseleave", (event, d) => { mouseLeaved(event) })
                // 右クリックした時
                .on("contextmenu", (event, d) => {
                    //event.preventDefault();
                    /* doSomething */
                });
        }
    });
    // --------------------ここまでsquashed-arc用--------------------
}


// 左クリックされた時の動作
//
// p: クリックされた円弧or円のデータ
function leftClicked(p) {

    // pがnull（parentがnullの時にクリックされた）の場合はリターンして何もしない
    if (p == null) return;

    // circleのデータを更新
    svgElement.select("circle")
        .datum(p);

    // Listの更新
    updateList(p);

    // クリックされた円弧の移動先を設定
    // "each"によって全てのノードについて設定を行う
    //
    // p: 移動前のノードデータ
    // d: 移動後のノードデータ
    partition.each(d => d.target = {
        x0: Math.max(0, Math.min(1, (d.x0 - p.x0) / (p.x1 - p.x0))) * 2 * Math.PI,
        x1: Math.max(0, Math.min(1, (d.x1 - p.x0) / (p.x1 - p.x0))) * 2 * Math.PI,
        y0: Math.max(0, d.y0 - p.depth),
        y1: Math.max(0, d.y1 - p.depth)
    });

    // 移動先のノードの深さ
    const targetDepth = p.depth;

    // "path", "text"の要素を全て削除
    svgElement.selectAll("path").remove();
    svgElement.selectAll("text").remove();

    // Arcを描画
    drawArc(targetDepth, false);

    // FileSizeを描画
    drawFileSize(p.value);

    // 変更を反映させる
    svgElement.enter();

    // --------------------ここからsquashed-arc用--------------------
    svgElement.selectAll("path.squashed-arc")
        .attr("fill-opacity", 0);
    // --------------------ここまでsquashed-arc用--------------------
    // --------------------ここからtext用--------------------
    svgElement.selectAll("text")
        .attr("fill-opacity", 0);
    // --------------------ここまでtext用--------------------

    // --------------------ここからmain-arc用--------------------
    // d.current: 移動前の円弧の座標
    // d.target: 移動後の円弧の座標
    svgElement.selectAll("path.main-arc")
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
            svgElement.selectAll("path.squashed-arc")
                .transition()
                .duration(transitionDuration / 3)
                .ease(d3.easeLinear)
                .attr("fill-opacity", 1);
            // --------------------ここまでsquashed-arc用--------------------
            // --------------------ここからtext用--------------------
            svgElement.selectAll("text")
                .transition()
                .duration(transitionDuration / 3)
                .ease(d3.easeLinear)
                .attr("fill-opacity", 1);
            // --------------------ここまでtext用--------------------
        });
    // --------------------ここまでmain-arc用--------------------
}


// タイマーハンドラー
let timerHandler = null;
// 他のアニメーションをinterruptしないように、transitionにnameを設定する
const transitionName = "blink"


// カーソルを合わせた時の動作
//
// event: イベントハンドラー
// p: カーソルを合わせた円弧or円のデータ
function mouseEntered(event, p) {
    // カーソルを合わせた円弧or円のパスを取得
    const target = d3.select(event.currentTarget);

    // アニメーションをリピート
    repeat();

    // タイマーをセット
    timerHandler = setTimeout(() => {
        // Listの更新
        updateList(p);
    }, hoverTimeout);

    // リピート用関数
    function repeat() {
        target
            .transition(transitionName)
            .duration(blinkInterval / 2)
            .ease(d3.easeLinear)
            .attr("fill-opacity", 0.5)
            .transition(transitionName)
            .duration(blinkInterval / 2)
            .ease(d3.easeLinear)
            .attr("fill-opacity", 1)
            .on("end", repeat);
    };
}


// カーソルを離した時の動作
//
// event: イベントハンドラー
function mouseLeaved(event) {
    // カーソルを離した円弧or円のパスを取得
    const target = d3.select(event.currentTarget);

    // タイマーをキャンセル
    clearTimeout(timerHandler);

    // アニメーションを中断
    target
        .interrupt(transitionName)
        .attr("fill-opacity", 1);
}


// Listの更新
//
// node: ノードデータ
function updateList(node) {
    return props.viewDirectoryFileList.generateDirectoryList(node);
}


// 外部から参照可能なプロパティを定義
defineExpose({
    generateSunburst,
    leftClicked,
});

</script>

<template>
    <div ref="svgDOMRef" style="height: 70vmin;"></div>
</template>
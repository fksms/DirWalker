<script setup>

import { ref, watch } from "vue";

import * as d3 from "d3";

import { showContextMenu } from "./util";

// 親から渡されたコンポーネントの参照を受け取る
const props = defineProps(["viewDirectoryFileList", "viewBreadcrumbsList"]);


// DOM格納用
const svgDOM = ref();

// DOMへの参照
const svgDOMRef = ref();

// svgContainerの変更を監視して、描画を更新する
watch(svgDOM, (newValue) => {
    const container = svgDOMRef.value;
    if (container && newValue) {
        // 既存の描画をクリア
        container.innerHTML = "";
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
const hoverTimeout = 500;

// マウスホバーした際の点滅の間隔[msec]
const blinkInterval = 600;

// 中心円が消灯した時のカラーコード（背景色（blue-grey-darken-3）と同じ色）
const circleLightOffColorCodes = "#37474F";

// 中心円が点灯した時のカラーコード（blue-grey-lighten-3）
const circleLightOnColorCodes = "#B0BEC5";

// ディレクトリのカラーコード（原色）
const directoryColorCodes = ["#FF0000", "#FF8000", "#FFFF00", "#80FF00", "#00FF00", "#00FF80", "#00FFFF", "#0080FF", "#0000FF", "#8000FF", "#FF00FF", "#FF0080"];

// ルートディレクトリのカラーコード
const rootDirectoryColorCode = "#FFFFFF00";

// ファイルのカラーコード
const fileColorCode = "#C6C6C6";

// squashされた部分のカラーコード
const squashedColorCode = "#777777";

// --------------------パラメータ--------------------

// --------------------グローバル--------------------

// Hierarchy data
let root = null;

// SVG element data
let svgElement = null;

// --------------------グローバル--------------------


// Sunburstの作成
function generateSunburst(data) {

    // カラースケールの作成
    //
    // scaleOrdinal: 配列の繰り返し設定を行う
    const colorWheel = d3.scaleOrdinal().range(directoryColorCodes);

    // HierarchyNodeの作成
    //
    // sum: childrenの要素が0のもののサイズのみを足しこんでHierarchyNodeを作成
    // sort: サイズを降順でソート
    root = d3.hierarchy(data)
        .sum(d => d.children.length ? 0 : d.size)
        .sort((a, b) => b.value - a.value);

    // nodeId用カウンター
    let count = 0;

    // rootにパーティションデータを追加
    // （幅は2π、高さは1の長方形とする。）
    //
    // (x0, y0): 左上の座標
    // (x1, y1): 右下の座標
    root.x0 = 0;
    root.x1 = 2 * Math.PI;
    root.y0 = 0;
    root.y1 = 1;

    // 各ノードにプロパティを追加する
    //
    // each: ノードを幅優先で呼び出す
    root.each(d => {

        // nodeIdの追加（SVGのidは数字から始まってはいけないため、先頭に文字を付ける）
        // https://stackoverflow.com/questions/58302561/howto-select-an-element-by-its-id-d3
        d.nodeId = "M" + count;
        count++;

        // childrenを持っている場合、childrenの各valueに応じて、childrenそれぞれにパーティションデータを設定する。
        if (d.children) {
            d3.treemapDice(d, d.x0, d.depth + 1, d.x1, d.depth + 2);
        }

        // currentプロパティの追加
        d.current = {
            x0: d.x0,
            x1: d.x1,
            y0: d.y0,
            y1: d.y1
        };

        // targetプロパティの追加
        d.target = {
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
                d.color = rootDirectoryColorCode;
            }
            // depthが1の場合
            else if (d.depth == 1) {
                d.color = rgb2Hex(colorWheel(d.data.name)); // 原色
            }
            // depthが1より大きくvisibleDepth以下の場合
            else if (d.depth > 1 && d.depth <= visibleDepth) {

                // 考え方
                //
                // depth=1: 「原色->白」のカラーをX等分して、2つ目
                // depth=2: 「depth=1->白」のカラーをX-1等分して、2つ目
                // depth=3: 「depth=2->白」カラーをX-2等分して、2つ目
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
            d.color = fileColorCode;
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

    // Circleを更新
    updateCircle(root, true);

    // Arcを更新
    updateArc(root, true);

    // Listの更新
    updateList(root);

    // Breadcrumbsの更新
    updateBreadcrumbs(root);

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

    // DOMを格納
    svgDOM.value = svgElement.node();
}


// rgb形式からhex形式に変換
function rgb2Hex(rgb) {
    const hex = d3.color(rgb).formatHex();
    return hex.toUpperCase();
}


// TB/GB/MB/KBに変換
function toReadable(value) {
    if (value >= 1e12) {
        return [(value / 1e12).toFixed(1), "TB"];
    }
    else if (value >= 1e9) {
        return [(value / 1e9).toFixed(1), "GB"];
    }
    else if (value >= 1e6) {
        return [(value / 1e6).toFixed(1), "MB"];
    }
    else if (value >= 1e3) {
        return [(value / 1e3).toFixed(1), "KB"];
    }
    else {
        return [value.toFixed(1), "B"];
    }
}


// 円弧（arc）生成関数
function createArc() {
    // パーティションデータ（長方形）を円弧（バウムクーヘン形）に変換
    // （横の長さを2πと指定しているため、直接角度として変換できる）
    //
    // startAngle: 円弧の開始角度
    // endAngle: 円弧の終了角度
    // padAngle: 円弧間の間隔（パディング角度）
    // padRadius: 円弧間の間隔（パディング半径）
    // innerRadius: 円弧の内側の半径
    // outerRadius: 円弧の外側の半径
    const arcHeight = (width - radius * 2) / (visibleDepth * 2); // 円弧の外側の半径-円弧の内側の半径
    return d3.arc()
        .startAngle(d => d.x0)
        .endAngle(d => d.x1)
        .padAngle(0.01)
        .padRadius(radius)
        .innerRadius(d => radius + (d.y0 - 1) * arcHeight)
        .outerRadius(d => radius + (d.y1 - 1) * arcHeight - 1);
}


// 入力されたnodeが可視化される場合はtrue、可視化されない場合はfalseを返す。
//
// node: ノードデータ
function visualize(node) {
    return node.target.y0 > 0 && node.target.y0 <= visibleDepth && node.target.x1 > node.target.x0;
}


// Circleを更新
//
// node: ノードデータ
// isFirstCalled: 初めて呼ばれたか否か
function updateCircle(node, isFirstCalled) {

    if (isFirstCalled) {
        // 更新前に"circle"の要素を全て削除
        svgElement.selectAll("circle").remove();

        // 背景の円を設定
        // （背景の円を用意することで、手前の円の透過率を変化させた際に点滅しているように見える）
        svgElement.append("circle")
            // classを設定
            .classed("back", true)
            // 半径を設定（1だけ小さな値にする）
            .attr("r", radius - 1)
            // fill属性（塗りつぶし）を設定
            .attr("fill", circleLightOnColorCodes)
            // fill-opacity属性（塗りつぶしの透明度）を設定
            .attr("fill-opacity", 1);

        // 中心の円のパスを設定
        svgElement.append("circle")
            .datum(node)
            // classを設定
            .classed("front", true)
            // IDを設定
            .attr("id", node.nodeId)
            // 半径を設定
            .attr("r", radius)
            // fill属性（塗りつぶし）を設定
            .attr("fill", circleLightOffColorCodes)
            // fill-opacity属性（塗りつぶしの透明度）を設定
            .attr("fill-opacity", 1)
            // ポインターイベントの設定
            .attr("pointer-events", "all")
            // カーソルを指差しの手にする
            .style("cursor", "pointer")
            // カーソルを合わせた時
            .on("mouseenter", (event, d) => mouseEntered(event, d))
            // カーソルを離した時
            .on("mouseleave", (event, d) => mouseLeaved(event, d))
            // 左クリックした時
            .on("click", (event, d) => leftClicked(d.parent))
            // 右クリックした時
            .on("contextmenu", (event, d) => {
                event.preventDefault(); // デフォルトの動作をキャンセル
                rightClicked(d)
            });
    }
    else {
        // circleのデータを更新（手前の円（front）を指定する）
        svgElement.select("circle.front")
            .datum(node)
            // IDを設定
            .attr("id", node.nodeId);
    }

    // 更新前に"text"の要素を全て削除
    svgElement.selectAll("text").remove();

    // Textを設定（中心に表示されるファイルサイズ）
    svgElement.selectAll("text")
        .data(toReadable(node.value))
        .join("text")
        .attr("text-anchor", "middle")
        .attr("fill", "#FFFFFF")
        .attr("x", 0)
        .attr("y", (d, i) => i * 35 - 5)
        .attr("font-size", "2em")
        .attr("pointer-events", "none")
        .text(d => d);
}


// Arcを更新
//
// node: ノードデータ
// isFirstCalled: 初めて呼ばれたか否か
function updateArc(node, isFirstCalled) {

    // 更新前に"path"の要素を全て削除
    svgElement.selectAll("path").remove();

    // 円弧（arc）の生成
    const arc = createArc();

    // Mapオブジェクト
    const squashedArcs = new Map();

    // --------------------ここからmain-arc用--------------------
    // 指定した要素を全て選択
    svgElement.selectAll("path.main-arc")
        // データ配列を作成
        .data(root.descendants().filter(d => {

            // --------------------ここからプロパティの更新--------------------
            if (!isFirstCalled) {
                // 処理軽減のために、後続の処理で面積な小さなアークをパスから除外しており、
                // その結果として、除外されたアークはcurrentプロパティが更新されなくなる。
                // そのため、ここでcurrentプロパティの更新を行う。
                d.current = {
                    x0: d.target.x0,
                    x1: d.target.x1,
                    y0: d.target.y0,
                    y1: d.target.y1
                }

                // targetプロパティの更新
                d.target = {
                    x0: Math.max(0, Math.min(1, (d.x0 - node.x0) / (node.x1 - node.x0))) * 2 * Math.PI,
                    x1: Math.max(0, Math.min(1, (d.x1 - node.x0) / (node.x1 - node.x0))) * 2 * Math.PI,
                    y0: Math.max(0, d.y0 - node.depth),
                    y1: Math.max(0, d.y1 - node.depth)
                };
            }
            // --------------------ここまでプロパティの更新--------------------

            // 可視化されないものはパスから除外する
            if (!visualize(d)) return false;

            // 円弧の角度[degree]が小さいものは除外する
            if ((d.target.x1 - d.target.x0) < (angleThreshold * Math.PI / 180)) {

                // --------------------ここからsquashed-arc用--------------------
                const parentName = d.parent.data.name;

                // 除外された円弧のparentNameが、まだMapに格納されていない場合
                if (!squashedArcs.has(parentName)) {
                    // オブジェクトの作成
                    const squashedObject = {
                        parentNode: d.parent,
                        head: d.value, // squashされた部分で最もサイズの大きいノードのvalue
                        x0: d.target.x0,
                        x1: d.target.x1,
                        y0: d.target.y0,
                        y1: d.target.y1
                    }
                    // 除外された円弧のparentNameをキーとして、座標を格納
                    squashedArcs.set(parentName, squashedObject);
                }
                // 除外された円弧のparentNameが、既にMapに格納されている場合
                else {
                    // 格納された座標を取得
                    let tmpCoordinates = squashedArcs.get(parentName);
                    // 座標をUpdate
                    tmpCoordinates.x1 = d.target.x1;
                    // 座標を格納
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
        // IDを設定
        .attr("id", d => d.nodeId)
        // d属性（パス）を設定
        .attr("d", d => arc(d.target))
        // fill属性（塗りつぶし）を設定
        .attr("fill", d => d.color)
        // fill-opacity属性（塗りつぶしの透明度）を設定
        .attr("fill-opacity", d => visualize(d) ? 1 : 0)
        // ポインターイベントの設定
        .attr("pointer-events", d => visualize(d) ? "auto" : "none")
        // カーソルを指差しの手にする
        .style("cursor", "pointer")
        // カーソルを合わせた時
        .on("mouseenter", (event, d) => mouseEntered(event, d))
        // カーソルを離した時
        .on("mouseleave", (event, d) => mouseLeaved(event, d))
        // 左クリックした時
        .on("click", (event, d) => leftClicked(d))
        // 右クリックした時
        .on("contextmenu", (event, d) => {
            event.preventDefault(); // デフォルトの動作をキャンセル
            rightClicked(d)
        });
    // --------------------ここまでmain-arc用--------------------

    // --------------------ここからsquashed-arc用--------------------
    // 除外された円弧を圧縮したものを表示
    squashedArcs.forEach((value, key) => {
        // 円弧の角度[degree]が大きいもののみ表示
        if ((value.x1 - value.x0) > (angleThreshold * Math.PI / 180)) {

            // squashed-arc用座標を生成
            const squashedArcCoordinates = {
                x0: value.x0,
                x1: value.x1,
                y0: value.y0,
                y1: value.y1
            }

            // リスト生成時のオプションを指定
            const option = {
                color: squashedColorCode,
                threshold: value.head // リスト生成時の閾値
            }

            // path要素を生成
            svgElement.append("path")
                // データを作成
                .datum(value.parentNode)
                // classを設定
                .classed("squashed-arc", true)
                // IDを設定
                .attr("id", "_")
                // d属性（パス）を設定
                .attr("d", arc(squashedArcCoordinates))
                // fill属性（塗りつぶし）を設定
                .attr("fill", squashedColorCode)
                // カーソルを合わせた時
                .on("mouseenter", (event, d) => mouseEntered(event, d, option))
                // カーソルを離した時
                .on("mouseleave", (event, d) => mouseLeaved(event, d))
                // 右クリックした時
                .on("contextmenu", (event, d) => {
                    event.preventDefault(); // デフォルトの動作をキャンセル
                    //rightClicked(d)
                });
        }
    });
    // --------------------ここまでsquashed-arc用--------------------
}


// Sunburstの更新
//
// node: ノードデータ
function updateSunburst(node) {

    // Circleを更新
    updateCircle(node, false);

    // Arcを更新
    updateArc(node, false);

    // 円弧（arc）の生成
    const arc = createArc();

    // main-arcのアニメーション
    svgElement.selectAll("path.main-arc")
        .transition()
        .duration(transitionDuration)
        .tween("scale", d => {
            const i = d3.interpolate(d.current, d.target);
            return t => d.current = i(t);
        })
        .attrTween("d", d => () => arc(d.current));

    // squashed-arcのアニメーション
    svgElement.selectAll("path.squashed-arc")
        .attr("fill-opacity", 0);
    svgElement.selectAll("path.squashed-arc")
        .transition()
        .duration(transitionDuration)
        .ease(d3.easeExpIn)
        .attr("fill-opacity", 1);

    // textのアニメーション
    svgElement.selectAll("text")
        .attr("fill-opacity", 0);
    svgElement.selectAll("text")
        .transition()
        .duration(transitionDuration)
        .ease(d3.easeExpIn)
        .attr("fill-opacity", 1);
}


// Nodeを削除
//
// node: ノードデータ
function removeNode(node) {

    // 削除対象のノードのサイズ・ID・親ノードを保持
    const removedNodeSize = node.value;
    const removedNodeId = node.nodeId;
    const removedNodeParent = node.parent;

    // 中心のノードを保持
    let centerNode = null;

    // nodeがnullになるまで繰り返し（上方向に走査）
    while (node != null) {
        // targetのy0が0、y1が1の時に中心となる（必ず存在する）
        if (node.target.y0 == 0 && node.target.y1 == 1) {
            centerNode = node;
        }

        // 減算
        node.value = node.value - removedNodeSize;
        node.data.size = node.data.size - removedNodeSize;

        // childrenを持っている場合、childrenを降順で再ソート
        if (node.children) {
            node.children.sort((a, b) => b.value - a.value);
        }

        // 親に移動
        node = node.parent;
    }

    // 一旦親ノードに移動してから、該当の子ノードを消去する（横方向に走査）
    for (let i = 0; i < removedNodeParent.children.length; i++) {
        // 消去対象のノードIDと一致するかを確認
        if (removedNodeParent.children[i].nodeId == removedNodeId) {
            removedNodeParent.children.splice(i, 1);
            break;
        }
    }

    // 各ノードのプロパティ（x0, x1, y0, y1）を更新する
    //
    // each: ノードを幅優先で呼び出す
    root.each(d => {
        // childrenを持っている場合、childrenの各valueに応じて、childrenそれぞれにパーティションデータを設定する。
        if (d.children) {
            d3.treemapDice(d, d.x0, d.depth + 1, d.x1, d.depth + 2);
        }
    });

    // 中心のノードを削除した場合は親ノードを設定
    if (removedNodeId == centerNode.nodeId) {
        node = centerNode.parent;
    }
    // それ以外は中心のノードを維持
    else {
        node = centerNode;
    }

    // Sunburstの更新
    updateSunburst(node);

    // Listの更新
    updateList(removedNodeParent);

    // Breadcrumbsの更新
    updateBreadcrumbs(node);
}


// リスト更新用タイマーID
let timerId = null;

// 他のアニメーションをinterruptしないように、transitionにnameを設定する
const transitionName = "blink"


// カーソルを合わせた時の動作
//
// event: イベントハンドラー（List側から呼び出された場合、イベントハンドラーは無効となる）
// node: ノードデータ
// option: オプション
function mouseEntered(event, node, option = null) {

    // 円弧or円のパスを格納
    let targetElement = null;

    // イベントハンドラーが有効かを確認（List側から呼び出された場合、イベントハンドラーは無効となる）
    if (event) {
        // イベントハンドラーが有効な場合は、イベントハンドラーからパスを検索する
        targetElement = d3.select(event.currentTarget);

        // リスト更新用タイマーをセット（タイムアウト後に更新）
        timerId = setTimeout(() => {
            // Listの更新
            updateList(node, option);
        }, hoverTimeout);
    }
    else {
        // イベントハンドラーが無効の場合は、nodeIdによってパスを検索する
        targetElement = svgElement.select("#" + node.nodeId);
    }

    // アニメーションをリピート
    repeat();

    // リピート用関数
    function repeat() {
        targetElement
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
// event: イベントハンドラー（List側から呼び出された場合、イベントハンドラーは無効となる）
// node: ノードデータ
function mouseLeaved(event, node) {

    // 円弧or円のパスを格納
    let targetElement = null;

    // イベントハンドラーが有効かを確認（List側から呼び出された場合、イベントハンドラーは無効となる）
    if (event) {
        // イベントハンドラーが有効な場合は、イベントハンドラーからパスを検索する
        targetElement = d3.select(event.currentTarget);

        // リスト更新用タイマーをキャンセル
        clearTimeout(timerId);
    }
    else {
        // イベントハンドラーが無効の場合は、nodeIdによってパスを検索する
        targetElement = svgElement.select("#" + node.nodeId);
    }

    // アニメーションを中断
    targetElement
        .interrupt(transitionName)
        .attr("fill-opacity", 1);
}


// 左クリックされた時の動作
//
// node: ノードデータ
function leftClicked(node) {

    // 自身がnullの場合はリターンして何もしない（parentがnullの時にクリックされた時）
    if (node == null) return;

    // childrenがnullの場合はリターンして何もしない（ファイルor空のディレクトリ）
    if (node.children == null) return;

    // valueが0の場合はリターンして何もしない（サイズが0バイトのディレクトリ）
    if (node.value == 0) return;

    // リスト更新用タイマーをキャンセル
    clearTimeout(timerId);

    // Sunburstの更新
    updateSunburst(node);

    // Listの更新
    updateList(node);

    // Breadcrumbsの更新
    updateBreadcrumbs(node);
}


// 右クリックされた時の動作
//
// node: ノードデータ
function rightClicked(node) {

    // 自身がnullの場合はリターンして何もしない（parentがnullの時にクリックされた時）
    if (node == null) return;

    // コンテキストメニューを表示（"removeNode"をコールバック関数として渡してあげる）
    showContextMenu(node, removeNode);
}


// Listの更新
//
// node: ノードデータ
// option: オプション
function updateList(node, option = null) {
    return props.viewDirectoryFileList.generateDirectoryList(node, option);
}


// Breadcrumbsの更新
//
// node: ノードデータ
function updateBreadcrumbs(node) {
    return props.viewBreadcrumbsList.generateBreadcrumbs(node);
}


// 外部から参照可能なプロパティを定義
defineExpose({
    generateSunburst,
    leftClicked,
    rightClicked,
    mouseEntered,
    mouseLeaved,
    toReadable,
});

</script>

<template>
    <div ref="svgDOMRef" style="height: 60vmin;"></div>
</template>
#!/bin/sh

wsm="./rs-japath2tokens.wasm"

filepaths(){
	echo /usr/share/zoneinfo
	echo Documents/2026/04/13/2026年度UIテストシート.xlsm
	echo Documents/2026/04/スマホ決済画面のクレカ情報入力時の内部サーバエラー.md
	echo Documents/2026/次期基幹システム刷新プロジェクト要件定義書における営業部指摘事項修正版.md
}

filepaths |
	wasmtime run "${wsm}" |
	jq -c

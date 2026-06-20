/*
MIT License

Copyright (c) 2022 [these people](https://github.com/tw-in-js/twind/graphs/contributors)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
(function(g,f){typeof exports==='object'&&typeof module!=='undefined'?f(exports):typeof define==='function'&&define.amd?define(['exports'],f):(g=typeof globalThis!=='undefined'?globalThis:g||self,f(g.twindSheets={}));}(this,(function(exports){'use strict';var s=Object.assign;var i="__twind",l=t=>{let e=self[i];return e||(e=document.head.appendChild(document.createElement("style")),e.id=i,t&&(e.nonce=t),e.appendChild(document.createTextNode(""))),e};var g=({nonce:t,target:e=l(t)}={})=>{let r=e.childNodes.length;return {target:e,insert:(n,o)=>e.insertBefore(document.createTextNode(n),e.childNodes[r+o])}},S=()=>{let t=[],e=[],r=(n,o)=>e[o]=n(e[o]);return {init:n=>r(n,t.push(n)-1),reset:(n=[])=>([n,e]=[e,n],t.forEach(r),n)}},d=()=>{let t=S(),e;return t.init((r=[])=>e=r),Object.defineProperties({get target(){return [...e]},insert:(r,n)=>e.splice(n,0,r)},Object.getOwnPropertyDescriptors(t))},c=t=>({id:i,textContent:(Array.isArray(t)?t:t.target).join("")}),p=(t,e)=>{let{id:r,textContent:n}=c(t);return e=s(s({},e),{id:r}),`<style${Object.keys(e).reduce((o,a)=>`${o} ${a}=${JSON.stringify(e[a])}`,"")}>${n}</style>`};exports.domSheet=g;exports.getStyleTag=p;exports.getStyleTagProperties=c;exports.virtualSheet=d;Object.defineProperty(exports,'__esModule',{value:true});})));//# sourceMappingURL=sheets.umd.js.map

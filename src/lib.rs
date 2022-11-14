// Copyright (c) 2021 RSBadges Authors
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
// 1. Redistributions of source code must retain the above copyright notice,
//    this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its contributors
//    may be used to endorse or promote products derived from this software
//    without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
// INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA,
// OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
// OF SUCH DAMAGE.

//! Create code badges from the comfort and safety of Rust
//!
//! *Flat*
//!
//! <svg xmlns="http://www.w3.org/2000/svg"  xmlns:xlink="http://www.w3.org/1999/xlink" width="66.8041" height="20" role="img" aria-label="style: flat">      <linearGradient id="smoothLmlydMy" x2="0" y2="100%">    <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>    <stop offset="1" stop-opacity=".1"/>  </linearGradient>  <clipPath id="roundLmlydMy">    <rect width="66.8041" height="20" rx="3" fill="#fff"/>  </clipPath>  <g clip-path="url(#roundLmlydMy)">    <rect width="21.451025" height="20" fill="rgb(85, 85, 85)">                </rect>    <rect x="21.451025" width="45.353073" height="20" fill="rgb(178, 34, 34)">                </rect>    <rect width="66.8041" height="20" fill="url(#smoothLmlydMy)"/>  </g>  <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110">            <text aria-hidden="true" x="117.25513" y="150" fill="#010101" fill-opacity=".3" transform="scale(0.1)" textLength="114.510254" lengthAdjust="spacing">版</text>    <text x="117.25513" y="140" transform="scale(0.1)" textLength="114.510254" lengthAdjust="spacing">版</text>    <text aria-hidden="true" x="431.27563" y="150" fill="#010101" fill-opacity=".3" transform="scale(0.1)" textLength="353.53073" lengthAdjust="spacing">不知道</text>    <text x="431.27563" y="140" transform="scale(0.1)" textLength="353.53073" lengthAdjust="spacing">不知道</text>                              </g></svg>
//!
//! *FlatSquare*
//!
//! <svg xmlns="http://www.w3.org/2000/svg"  xmlns:xlink="http://www.w3.org/1999/xlink" width="149.01678" height="20" role="img" aria-label="style: flat-square">      <g shape-rendering="crispEdges">    <rect width="57.80707" height="20" fill="rgb(34, 139, 34)">                </rect>    <rect x="57.80707" width="91.20972" height="20" fill="rgb(0, 126, 198)">                </rect>  </g>  <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110">            <text x="299.03537" y="140" transform="scale(0.1)" fill="#fff" textLength="478.0707">link here</text>    <text x="1024.1193" y="140" transform="scale(0.1)" fill="#fff" textLength="812.09717">and a link here</text>                <a xlink:href="http:&#x2f;&#x2f;www.tangramvision.com">      <rect width="57.80707" height="20" fill="rgba(0,0,0,0)"/>    </a>            <a xlink:href="http:&#x2f;&#x2f;www.crates.io">      <rect x="57.80707" width="91.20972" height="20" fill="rgba(0,0,0,0)"/>    </a>          </g></svg>
//!
//! *Plastic*
//!
//! <svg xmlns="http://www.w3.org/2000/svg"  xmlns:xlink="http://www.w3.org/1999/xlink" width="90" height="18" role="img" aria-label="style: plastic">      <linearGradient id="smoothX2N04nc" x2="0" y2="100%">    <stop offset="0" stop-color="#fff" stop-opacity=".7"/>    <stop offset=".1" stop-color="#aaa" stop-opacity=".1"/>    <stop offset=".9" stop-color="#000" stop-opacity=".3"/>    <stop offset="1" stop-color="#000" stop-opacity=".5"/>  </linearGradient>  <clipPath id="roundX2N04nc">    <rect width="90" height="18" rx="3" fill="#fff"/>  </clipPath>  <g clip-path="url(#roundX2N04nc)">    <rect width="51" height="18" fill="rgb(85, 85, 85)">                </rect>    <rect x="51" width="39" height="18" fill="rgb(0, 126, 198)">                </rect>    <rect width="90" height="18" fill="url(#smoothX2N04nc)"/>  </g>  <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110">            <text aria-hidden="true" x="266" y="140" fill="#010101" fill-opacity=".3" transform="scale(0.1)" textLength="412" lengthAdjust="spacing">version</text>    <text x="266" y="130" transform="scale(0.1)" textLength="412" lengthAdjust="spacing">version</text>    <text aria-hidden="true" x="697" y="140" fill="#010101" fill-opacity=".3" transform="scale(0.1)" textLength="290" lengthAdjust="spacing">1.2.3</text>    <text x="697" y="130" transform="scale(0.1)" textLength="290" lengthAdjust="spacing">1.2.3</text>                        </g></svg>
//!
//! *ForTheBadge*
//!
//! <svg xmlns="http://www.w3.org/2000/svg"    xmlns:xlink="http://www.w3.org/1999/xlink" width="247.74521" height="28" role="img" aria-label="STYLE: FOR-THE-BADGE">            <g shape-rendering="crispEdges">        <rect width="105.69821" height="28" fill="rgb(85, 85, 85)">                                </rect>        <rect x="105.69821" width="142.047" height="28" fill="rgb(0, 0, 0)">                                </rect>    </g>    <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="100">                        <text x="528.4911" y="175" transform="scale(0.1)" fill="#fff" textLength="816.9821">RÖCK DÖTS</text>        <text x="1767.217" y="175" font-weight="bold" transform="scale(0.1)" fill="#fff" textLength="1180.47">VERY METAL INDEED</text>                                                            </g></svg>
//!
//! *Social*
//!
//! <svg xmlns="http://www.w3.org/2000/svg"    xmlns:xlink="http://www.w3.org/1999/xlink" width="146.27295" height="20" role="img" aria-label="Style: social">            <style>a:hover #llink{fill:url(#roundG0zlUED);stroke:#ccc}a:hover #rlink{fill:#4183c4}</style>    <linearGradient id="smoothG0zlUED" x2="0" y2="100%">        <stop offset="0" stop-color="#fcfcfc" stop-opacity="0"/>        <stop offset="1" stop-opacity=".1"/>    </linearGradient>    <linearGradient id="roundG0zlUED" x2="0" y2="100%">        <stop offset="0" stop-color="#ccc" stop-opacity=".1"/>        <stop offset="1" stop-opacity=".1"/>    </linearGradient>    <g stroke="#d5d5d5">        <rect stroke="none" fill="#fcfcfc" x="0.5" y="0.5" width="50.217674" height="19" rx="2"/>                <rect x="56.717674" y="0.5" width="89.055275" height="19" rx="2" fill="#fafafa"/>        <rect x="56.217674" y="7.5" width="0.5" height="5" stroke="#fafafa"/>        <path d="M56.717674 6.5 l-3 3v1 l3 3" stroke="d5d5d5" fill="#fafafa"/>            </g>        <image x="5" y="3" width="14" height="14" xlink:href="data:image&#x2f;svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDI0IDI0Ij48dGl0bGU+UnVzdCBpY29uPC90aXRsZT48cGF0aCBkPSJNMjMuODM0NiAxMS43MDMzbC0xLjAwNzMtLjYyMzZhMTMuNzI2OCAxMy43MjY4IDAgMDAtLjAyODMtLjI5MzZsLjg2NTYtLjgwNjlhLjM0ODMuMzQ4MyAwIDAwLS4xMTU0LS41NzhsLTEuMTA2Ni0uNDE0YTguNDk1OCA4LjQ5NTggMCAwMC0uMDg3LS4yODU2bC42OTA0LS45NTg3YS4zNDYyLjM0NjIgMCAwMC0uMjI1Ny0uNTQ0NmwtMS4xNjYzLS4xODk0YTkuMzU3NCA5LjM1NzQgMCAwMC0uMTQwNy0uMjYyMmwuNDktMS4wNzYxYS4zNDM3LjM0MzcgMCAwMC0uMDI3NC0uMzM2MS4zNDg2LjM0ODYgMCAwMC0uMzAwNi0uMTU0bC0xLjE4NDUuMDQxNmE2Ljc0NDQgNi43NDQ0IDAgMDAtLjE4NzMtLjIyNjhsLjI3MjMtMS4xNTNhLjM0NzIuMzQ3MiAwIDAwLS40MTctLjQxNzJsLTEuMTUzMi4yNzI0YTE0LjAxODMgMTQuMDE4MyAwIDAwLS4yMjc4LS4xODczbC4wNDE1LTEuMTg0NWEuMzQ0Mi4zNDQyIDAgMDAtLjQ5LS4zMjhsLTEuMDc2LjQ5MWMtLjA4NzItLjA0NzYtLjE3NDItLjA5NTItLjI2MjMtLjE0MDdsLS4xOTAzLTEuMTY3M0EuMzQ4My4zNDgzIDAgMDAxNi4yNTYuOTU1bC0uOTU5Ny42OTA1YTguNDg2NyA4LjQ4NjcgMCAwMC0uMjg1NS0uMDg2bC0uNDE0LTEuMTA2NmEuMzQ4My4zNDgzIDAgMDAtLjU3ODEtLjExNTRsLS44MDY5Ljg2NjZhOS4yOTM2IDkuMjkzNiAwIDAwLS4yOTM2LS4wMjg0TDEyLjI5NDYuMTY4M2EuMzQ2Mi4zNDYyIDAgMDAtLjU4OTIgMGwtLjYyMzYgMS4wMDczYTEzLjczODMgMTMuNzM4MyAwIDAwLS4yOTM2LjAyODRMOS45ODAzLjMzNzRhLjM0NjIuMzQ2MiAwIDAwLS41NzguMTE1NGwtLjQxNDEgMS4xMDY1Yy0uMDk2Mi4wMjc0LS4xOTAzLjA1NjctLjI4NTUuMDg2TDcuNzQ0Ljk1NWEuMzQ4My4zNDgzIDAgMDAtLjU0NDcuMjI1OEw3LjAwOSAyLjM0OGE5LjM1NzQgOS4zNTc0IDAgMDAtLjI2MjIuMTQwN2wtMS4wNzYyLS40OTFhLjM0NjIuMzQ2MiAwIDAwLS40OS4zMjhsLjA0MTYgMS4xODQ1YTcuOTgyNiA3Ljk4MjYgMCAwMC0uMjI3OC4xODczTDMuODQxMyAzLjQyNWEuMzQ3Mi4zNDcyIDAgMDAtLjQxNzEuNDE3MWwuMjcxMyAxLjE1MzFjLS4wNjI4LjA3NS0uMTI1NS4xNTA5LS4xODYzLjIyNjhsLTEuMTg0NS0uMDQxNWEuMzQ2Mi4zNDYyIDAgMDAtLjMyOC40OWwuNDkxIDEuMDc2MWE5LjE2NyA5LjE2NyAwIDAwLS4xNDA3LjI2MjJsLTEuMTY2Mi4xODk0YS4zNDgzLjM0ODMgMCAwMC0uMjI1OC41NDQ2bC42OTA0Ljk1ODdhMTMuMzAzIDEzLjMwMyAwIDAwLS4wODcuMjg1NWwtMS4xMDY1LjQxNGEuMzQ4My4zNDgzIDAgMDAtLjExNTUuNTc4MWwuODY1Ni44MDdhOS4yOTM2IDkuMjkzNiAwIDAwLS4wMjgzLjI5MzVsLTEuMDA3My42MjM2YS4zNDQyLjM0NDIgMCAwMDAgLjU4OTJsMS4wMDczLjYyMzZjLjAwOC4wOTgyLjAxODIuMTk2NC4wMjgzLjI5MzZsLS44NjU2LjgwNzlhLjM0NjIuMzQ2MiAwIDAwLjExNTUuNTc4bDEuMTA2NS40MTQxYy4wMjczLjA5NjIuMDU2Ny4xOTE0LjA4Ny4yODU1bC0uNjkwNC45NTg3YS4zNDUyLjM0NTIgMCAwMC4yMjY4LjU0NDdsMS4xNjYyLjE4OTNjLjA0NTYuMDg4LjA5MjIuMTc1MS4xNDA4LjI2MjJsLS40OTEgMS4wNzYyYS4zNDYyLjM0NjIgMCAwMC4zMjguNDlsMS4xODM0LS4wNDE1Yy4wNjE4LjA3NjkuMTIzNS4xNTI4LjE4NzMuMjI3N2wtLjI3MTMgMS4xNTQxYS4zNDYyLjM0NjIgMCAwMC40MTcxLjQxNjFsMS4xNTMtLjI3MTNjLjA3NS4wNjM4LjE1MS4xMjU1LjIyNzkuMTg2M2wtLjA0MTUgMS4xODQ1YS4zNDQyLjM0NDIgMCAwMC40OS4zMjdsMS4wNzYxLS40OWMuMDg3LjA0ODYuMTc0MS4wOTUxLjI2MjIuMTQwN2wuMTkwMyAxLjE2NjJhLjM0ODMuMzQ4MyAwIDAwLjU0NDcuMjI2OGwuOTU4Ny0uNjkwNGE5LjI5OSA5LjI5OSAwIDAwLjI4NTUuMDg3bC40MTQgMS4xMDY2YS4zNDUyLjM0NTIgMCAwMC41NzgxLjExNTRsLjgwNzktLjg2NTZjLjA5NzIuMDExMS4xOTU0LjAyMDMuMjkzNi4wMjk0bC42MjM2IDEuMDA3M2EuMzQ3Mi4zNDcyIDAgMDAuNTg5MiAwbC42MjM2LTEuMDA3M2MuMDk4Mi0uMDA5MS4xOTY0LS4wMTgzLjI5MzYtLjAyOTRsLjgwNjkuODY1NmEuMzQ4My4zNDgzIDAgMDAuNTc4LS4xMTU0bC40MTQxLTEuMTA2NmE4LjQ2MjYgOC40NjI2IDAgMDAuMjg1NS0uMDg3bC45NTg3LjY5MDRhLjM0NTIuMzQ1MiAwIDAwLjU0NDctLjIyNjhsLjE5MDMtMS4xNjYyYy4wODgtLjA0NTYuMTc1MS0uMDkzMS4yNjIyLS4xNDA3bDEuMDc2Mi40OWEuMzQ3Mi4zNDcyIDAgMDAuNDktLjMyN2wtLjA0MTUtMS4xODQ1YTYuNzI2NyA2LjcyNjcgMCAwMC4yMjY3LS4xODYzbDEuMTUzMS4yNzEzYS4zNDcyLjM0NzIgMCAwMC40MTcxLS40MTZsLS4yNzEzLTEuMTU0MmMuMDYyOC0uMDc0OS4xMjU1LS4xNTA4LjE4NjMtLjIyNzhsMS4xODQ1LjA0MTVhLjM0NDIuMzQ0MiAwIDAwLjMyOC0uNDlsLS40OS0xLjA3NmMuMDQ3NS0uMDg3Mi4wOTUxLS4xNzQyLjE0MDctLjI2MjNsMS4xNjYyLS4xODkzYS4zNDgzLjM0ODMgMCAwMC4yMjU4LS41NDQ3bC0uNjkwNC0uOTU4Ny4wODctLjI4NTUgMS4xMDY2LS40MTRhLjM0NjIuMzQ2MiAwIDAwLjExNTQtLjU3ODFsLS44NjU2LS44MDc5Yy4wMTAxLS4wOTcyLjAyMDItLjE5NTQuMDI4My0uMjkzNmwxLjAwNzMtLjYyMzZhLjM0NDIuMzQ0MiAwIDAwMC0uNTg5MnptLTYuNzQxMyA4LjM1NTFhLjcxMzguNzEzOCAwIDAxLjI5ODYtMS4zOTYuNzE0LjcxNCAwIDExLS4yOTk3IDEuMzk2em0tLjM0MjItMi4zMTQyYS42NDkuNjQ5IDAgMDAtLjc3MTUuNWwtLjM1NzMgMS42Njg1Yy0xLjEwMzUuNTAxLTIuMzI4NS43Nzk1LTMuNjE5My43Nzk1YTguNzM2OCA4LjczNjggMCAwMS0zLjY5NTEtLjgxNGwtLjM1NzQtMS42Njg0YS42NDguNjQ4IDAgMDAtLjc3MTQtLjQ5OWwtMS40NzMuMzE1OGE4LjcyMTYgOC43MjE2IDAgMDEtLjc2MTMtLjg5OGg3LjE2NzZjLjA4MSAwIC4xMzU2LS4wMTQxLjEzNTYtLjA4OHYtMi41MzZjMC0uMDc0LS4wNTM2LS4wODgxLS4xMzU2LS4wODgxaC0yLjA5NjZ2LTEuNjA3N2gyLjI2NzdjLjIwNjUgMCAxLjEwNjUuMDU4NyAxLjM5NCAxLjIwODguMDkwMS4zNTMzLjI4NzUgMS41MDQ0LjQyMzIgMS44NzI5LjEzNDYuNDEzLjY4MzMgMS4yMzgxIDEuMjY4NSAxLjIzODFoMy41NzE2YS43NDkyLjc0OTIgMCAwMC4xMjk2LS4wMTMxIDguNzg3NCA4Ljc4NzQgMCAwMS0uODExOS45NTI2ek02LjgzNjkgMjAuMDI0YS43MTQuNzE0IDAgMTEtLjI5OTctMS4zOTYuNzE0LjcxNCAwIDAxLjI5OTcgMS4zOTZ6TTQuMTE3NyA4Ljk5NzJhLjcxMzcuNzEzNyAwIDExLTEuMzA0LjU3OTEuNzEzNy43MTM3IDAgMDExLjMwNC0uNTc5em0tLjgzNTIgMS45ODEzbDEuNTM0Ny0uNjgyNGEuNjUuNjUgMCAwMC4zMy0uODU4NWwtLjMxNTgtLjcxNDdoMS4yNDMydjUuNjAyNUgzLjU2NjlhOC43NzUzIDguNzc1MyAwIDAxLS4yODM0LTMuMzQ4em02LjczNDMtLjU0MzdWOC43ODM2aDIuOTYwMWMuMTUzIDAgMS4wNzkyLjE3NzIgMS4wNzkyLjg2OTcgMCAuNTc1LS43MTA3Ljc4MTUtMS4yOTQ4Ljc4MTV6bTEwLjc1NzQgMS40ODYyYzAgLjIxODctLjAwOC40MzYzLS4wMjQzLjY1MWgtLjljLS4wOSAwLS4xMjY1LjA1ODYtLjEyNjUuMTQ3N3YuNDEzYzAgLjk3My0uNTQ4NyAxLjE4NDYtMS4wMjk2IDEuMjM4Mi0uNDU3Ni4wNTE3LS45NjQ4LS4xOTEzLTEuMDI3NS0uNDcxNy0uMjcwNC0xLjUxODYtLjcxOTgtMS44NDM2LTEuNDMwNS0yLjQwMzQuODgxNy0uNTU5OSAxLjc5OS0xLjM4NiAxLjc5OS0yLjQ5MTUgMC0xLjE5MzYtLjgxOS0xLjk0NTgtMS4zNzY5LTIuMzE1My0uNzgyNS0uNTE2My0xLjY0OTEtLjYxOTUtMS44ODMtLjYxOTVINS40NjgyYTguNzY1MSA4Ljc2NTEgMCAwMTQuOTA3LTIuNzY5OWwxLjA5NzQgMS4xNTFhLjY0OC42NDggMCAwMC45MTgyLjAyMTNsMS4yMjctMS4xNzQzYTguNzc1MyA4Ljc3NTMgMCAwMTYuMDA0NCA0LjI3NjJsLS44NDAzIDEuODk4MmEuNjUyLjY1MiAwIDAwLjMzLjg1ODVsMS42MTc4LjcxODhjLjAyODMuMjg3NS4wNDI1LjU3Ny4wNDI1Ljg3MTd6bS05LjMwMDYtOS41OTkzYS43MTI4LjcxMjggMCAxMS45ODQgMS4wMzE2LjcxMzcuNzEzNyAwIDAxLS45ODQtMS4wMzE2em04LjMzODkgNi43MWEuNzEwNy43MTA3IDAgMDEuOTM5NS0uMzYyNS43MTM3LjcxMzcgMCAxMS0uOTQwNS4zNjM1eiIvPjwvc3ZnPg=="/>        <g aria-hidden="true" fill="#333" text-anchor="middle" font-family="Helvetica Neue,Helvetica,Arial,sans-serif" text-rendering="geometricPrecision" font-weight="700" font-size="110px" line-height="14px">        <rect id="llink" stroke="#d5d5d5" fill="url(#smoothG0zlUED)" x=".5" y=".5" width="50.217674" height="19" rx="2" />        <text aria-hidden="true" x="336.08838" y="150" fill="#fff" transform="scale(.1)" textLength="232.17676">Rust</text>        <text x="336.08838" y="140" transform="scale(.1)" textLength="232.17676">Rust</text>        <text aria-hidden="true" x="1007.4531" y="150" fill="#fff" transform="scale(.1)" textLength="810.55273">so hot right now</text>        <text id="rlink" x="1007.4531" y="140" transform="scale(.1)" textLength="810.55273">so hot right now</text>    </g>                            </svg>
//!
//! ------------
//!
//! RSBadges is a Rust-friendly badge generator. The interface strives to be minimal
//! while still providing a feature-rich API. Both the label (the left side) and the
//! message (the right side) of the badge can be customized fully, with the ability to
//!
//! - Set text
//! - Set color using any valid CSS color code
//! - Embed a link into each side or a link for the whole badge
//! - Add a logo (in SVG format) from a local source or a URL
//! - Embed that logo's data into the badge directly
//! - Set the style of badge, as described in [Shields.io](http://shields.io)
//!
//! RSBadges can be used as an API or a command line interface (CLI). See the [Badge] and [Style]
//! docs for more details on arguments and capabilities.
//!
//! # API
//!
//! First, instantiate a [Badge] struct to set all of the generic options for a badge SVG.
//! This fully-populated Badge is then wrapped in a [Style] enum, which indicates which
//! style of badge to eventually generate.
//!
//! ```
//! use rsbadges::{Badge, Style};
//! let badge = Badge {
//!     label_text: String::from("Custom_label"),
//!     msg_text: String::from("Custom_msg"),
//!     label_color: String::from("red"),
//!     ..Badge::default()
//! };
//! // Create a plastic badge using the data created above.
//! let badge_style = Style::Plastic(badge);
//! ```
//!
//! Badge and Style together are sufficient to
//! tell RSBadges how to construct the right badge, which it does through `generate_svg()`:
//!
//! ```
//! # use rsbadges::{Badge, Style};
//! # let badge = Badge {
//! #     label_text: String::from("Custom_label"),
//! #     msg_text: String::from("Custom_msg"),
//! #     label_color: String::from("red"),
//! #     ..Badge::default()
//! # };
//! # // Create a plastic badge using the data created above.
//! # let badge_style = Style::Plastic(badge);
//! let badge_svg = badge_style.generate_svg().unwrap();
//! ```
//!
//! The resulting SVG String can be saved to file using the convenience function `save_svg()`:
//!
//! ```
//! # use rsbadges::{Badge, Style};
//! # let badge = Badge {
//! #     label_text: String::from("Custom_label"),
//! #     msg_text: String::from("Custom_msg"),
//! #     label_color: String::from("red"),
//! #     ..Badge::default()
//! # };
//! # // Create a plastic badge using the data created above.
//! # let badge_style = Style::Plastic(badge);
//! # let badge_svg = badge_style.generate_svg().unwrap();
//! rsbadges::save_svg("~/Downloads/badge.svg", &badge_svg);
//! ```
//!
//! See the [Badge] and [Style] documentation for more.
//!
//! # CLI
//!
//! The CLI features all of the customization options from the API, along with a
//! few quality-of-life improvements for command line use and evaluation, such as
//!
//! - Opening a created badge SVG in browser after creation
//! - Specifying a save directory for the SVG
//!
//! Valid argument formats match those found in the API (see [Badge]).
//! Don't worry if you get it wrong; RSBadges will let you know.
//!
//! | Short      | Long                                                      | Default
//! | ---------  | ------------------------------------                      | -------
//! | `-a`       | `--label <string>`                                        | "test"
//! | `-b`       | `--label-color <css_color>`                               | "#555"
//! | `-c`       | `--label-link <url>`                                      | ""
//! | `-x`       | `--msg <string>`                                          | "test"
//! | `-y`       | `--msg-color <css_color>`                                 | "#007ec6"
//! | `-z`       | `--msg-link <url>`                                        | ""
//! | `-l`       | `--logo <url or local path>`                              | ""
//! | `-f`       | `--save-to-svg-at <filepath/file.svg>`                    | ""
//! | `-s`       | `--style <plastic,flat,flatsquare,forthebadge,social>`    | "flat"
//! | `-o`       | `--open-in-browser`                                       | false
//! | `-h`       | `--help`                                                  | false
//! | `-e`       | `--embed-logo`                                            | false
//!
//! Run the CLI with the `-h` flag to see all possible arguments and flags.
//!

#![warn(missing_docs)] // warn if there are missing docs

mod badge;

pub use badge::{Badge, BadgeError, Style};
use std::fs;
use std::path::Path;

/// A convenience function to save an SVG to a file.
///
/// # Errors
///
/// This will return [BadgeError::CannotSaveToFile] if the directory
/// is malformed or cannot be accessed.
pub fn save_svg(filepath: &str, svg: &str) -> Result<(), BadgeError> {
    let svg_path = Path::new(filepath);
    if let Err(c) = fs::write(svg_path, svg) {
        println!("Error: {}", c);
        return Err(BadgeError::CannotSaveToFile(String::from(filepath)));
    }
    Ok(())
}

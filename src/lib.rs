// BSD 3-Clause License
//
// Copyright (c) 2021 RSBadges Authors
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
//    this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
//    may be used to endorse or promote products derived from this software
//    without specific prior written permission.
//
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
//! <svg xmlns="http://www.w3.org/2000/svg"  xmlns:xlink="http://www.w3.org/1999/xlink" width="67.42813" height="20" role="img" aria-label="style: flat">    <linearGradient id="smooth4tpEmXy" x2="0" y2="100%">    <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>    <stop offset="1" stop-opacity=".1"/>  </linearGradient>  <clipPath id="round4tpEmXy">    <rect width="67.42813" height="20" rx="3" fill="#fff"/>  </clipPath>  <g clip-path="url(#round4tpEmXy)">    <rect width="19.853168" height="20" fill="rgb(85, 85, 85)">          </rect>    <rect x="19.853168" width="47.574963" height="20" fill="rgb(178, 34, 34)">          </rect>    <rect width="67.42813" height="20" fill="url(#smooth4tpEmXy)"/>  </g>  <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110">        <text aria-hidden="true" x="109.26584" y="150" fill="#010101" fill-opacity=".3" transform="scale(0.1)" textLength="98.531685" lengthAdjust="spacing">版</text>    <text x="109.26584" y="140" transform="scale(0.1)" textLength="98.531685" lengthAdjust="spacing">版</text>    <text aria-hidden="true" x="426.4065" y="150" fill="#010101" fill-opacity=".3" transform="scale(0.1)" textLength="375.74963" lengthAdjust="spacing">不 知 道</text>    <text x="426.4065" y="140" transform="scale(0.1)" textLength="375.74963" lengthAdjust="spacing">不 知 道</text>                  </g></svg>
//!
//! *FlatSquare*
//!
//! <svg xmlns="http://www.w3.org/2000/svg"  xmlns:xlink="http://www.w3.org/1999/xlink" width="152.9915" height="20" role="img" aria-label="style: flat-square">    <g shape-rendering="crispEdges">    <rect width="59.344666" height="20" fill="rgb(34, 139, 34)">          </rect>    <rect x="59.344666" width="93.646835" height="20" fill="rgb(0, 126, 198)">          </rect>  </g>  <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110">        <text x="306.72333" y="140" transform="scale(0.1)" fill="#fff" textLength="493.44666">link here</text>    <text x="1051.6808" y="140" transform="scale(0.1)" fill="#fff" textLength="836.4684">and a link here</text>                  </g></svg>
//!
//! *Plastic*
//!
//! <svg xmlns="http://www.w3.org/2000/svg"  xmlns:xlink="http://www.w3.org/1999/xlink" width="88.855484" height="18" role="img" aria-label="style: plastic">    <linearGradient id="smoothTjMYf4A" x2="0" y2="100%">    <stop offset="0" stop-color="#fff" stop-opacity=".7"/>    <stop offset=".1" stop-color="#aaa" stop-opacity=".1"/>    <stop offset=".9" stop-color="#000" stop-opacity=".3"/>    <stop offset="1" stop-color="#000" stop-opacity=".5"/>  </linearGradient>  <clipPath id="roundTjMYf4A">    <rect width="88.855484" height="18" rx="3" fill="#fff"/>  </clipPath>  <g clip-path="url(#roundTjMYf4A)">    <rect width="51.752705" height="18" fill="rgb(85, 85, 85)">          </rect>    <rect x="51.752705" width="37.102783" height="18" fill="rgb(0, 126, 198)">          </rect>    <rect width="88.855484" height="18" fill="url(#smoothTjMYf4A)"/>  </g>  <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110">        <text aria-hidden="true" x="268.76352" y="140" fill="#010101" fill-opacity=".3" transform="scale(0.1)" textLength="417.52704" lengthAdjust="spacing">version</text>    <text x="268.76352" y="130" transform="scale(0.1)" textLength="417.52704" lengthAdjust="spacing">version</text>    <text aria-hidden="true" x="693.0409" y="140" fill="#010101" fill-opacity=".3" transform="scale(0.1)" textLength="271.0278" lengthAdjust="spacing">1.2.3</text>    <text x="693.0409" y="130" transform="scale(0.1)" textLength="271.0278" lengthAdjust="spacing">1.2.3</text>                  </g></svg>
//!
//! *ForTheBadge*
//!
//! <svg xmlns="http://www.w3.org/2000/svg"    xmlns:xlink="http://www.w3.org/1999/xlink" width="262.9521" height="28" role="img" aria-label="STYLE: FOR-THE-BADGE">        <g shape-rendering="crispEdges">        <rect width="97.72952" height="28" fill="rgb(85, 85, 85)">                    </rect>        <rect x="97.72952" width="165.22256" height="28" fill="rgb(0, 0, 0)">                    </rect>    </g>    <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="100">                <text x="488.6476" y="175" transform="scale(0.1)" fill="#fff" textLength="737.2952">RÖCK DÖTS</text>        <text x="1803.4081" y="175" font-weight="bold" transform="scale(0.1)" fill="#fff" textLength="1412.2256">VERY METAL INDEED</text>                                    </g></svg>
//!
//! *Social*
//!
//! <svg xmlns="http://www.w3.org/2000/svg"    xmlns:xlink="http://www.w3.org/1999/xlink" width="156.54895" height="20" role="img" aria-label="Style: social">        <style>a:hover #llink{fill:url(#roundzQkNqHB);stroke:#ccc}a:hover #rlink{fill:#4183c4}</style>    <linearGradient id="smoothzQkNqHB" x2="0" y2="100%">        <stop offset="0" stop-color="#fcfcfc" stop-opacity="0"/>        <stop offset="1" stop-opacity=".1"/>    </linearGradient>    <linearGradient id="roundzQkNqHB" x2="0" y2="100%">        <stop offset="0" stop-color="#ccc" stop-opacity=".1"/>        <stop offset="1" stop-opacity=".1"/>    </linearGradient>    <g stroke="#d5d5d5">        <rect stroke="none" fill="#fcfcfc" x="0.5" y="0.5" width="52.226604" height="19" rx="2"/>                <rect x="58.726604" y="0.5" width="97.32234" height="19" rx="2" fill="#fafafa"/>        <rect x="58.226604" y="7.5" width="0.5" height="5" stroke="#fafafa"/>        <path d="M58.726604 6.5 l-3 3v1 l3 3" stroke="d5d5d5" fill="#fafafa"/>            </g>        <image x="5" y="3" width="14" height="14" xlink:href="data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHZpZXdCb3g9IjAgMCAyNCAyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48dGl0bGU+UnVzdDwvdGl0bGU+PHBhdGggZD0iTTIzLjgzNDYgMTEuNzAzM2wtMS4wMDczLS42MjM2YTEzLjcyNjggMTMuNzI2OCAwIDAwLS4wMjgzLS4yOTM2bC44NjU2LS44MDY5YS4zNDgzLjM0ODMgMCAwMC0uMTE1NC0uNTc4bC0xLjEwNjYtLjQxNGE4LjQ5NTggOC40OTU4IDAgMDAtLjA4Ny0uMjg1NmwuNjkwNC0uOTU4N2EuMzQ2Mi4zNDYyIDAgMDAtLjIyNTctLjU0NDZsLTEuMTY2My0uMTg5NGE5LjM1NzQgOS4zNTc0IDAgMDAtLjE0MDctLjI2MjJsLjQ5LTEuMDc2MWEuMzQzNy4zNDM3IDAgMDAtLjAyNzQtLjMzNjEuMzQ4Ni4zNDg2IDAgMDAtLjMwMDYtLjE1NGwtMS4xODQ1LjA0MTZhNi43NDQ0IDYuNzQ0NCAwIDAwLS4xODczLS4yMjY4bC4yNzIzLTEuMTUzYS4zNDcyLjM0NzIgMCAwMC0uNDE3LS40MTcybC0xLjE1MzIuMjcyNGExNC4wMTgzIDE0LjAxODMgMCAwMC0uMjI3OC0uMTg3M2wuMDQxNS0xLjE4NDVhLjM0NDIuMzQ0MiAwIDAwLS40OS0uMzI4bC0xLjA3Ni40OTFjLS4wODcyLS4wNDc2LS4xNzQyLS4wOTUyLS4yNjIzLS4xNDA3bC0uMTkwMy0xLjE2NzNBLjM0ODMuMzQ4MyAwIDAwMTYuMjU2Ljk1NWwtLjk1OTcuNjkwNWE4LjQ4NjcgOC40ODY3IDAgMDAtLjI4NTUtLjA4NmwtLjQxNC0xLjEwNjZhLjM0ODMuMzQ4MyAwIDAwLS41NzgxLS4xMTU0bC0uODA2OS44NjY2YTkuMjkzNiA5LjI5MzYgMCAwMC0uMjkzNi0uMDI4NEwxMi4yOTQ2LjE2ODNhLjM0NjIuMzQ2MiAwIDAwLS41ODkyIDBsLS42MjM2IDEuMDA3M2ExMy43MzgzIDEzLjczODMgMCAwMC0uMjkzNi4wMjg0TDkuOTgwMy4zMzc0YS4zNDYyLjM0NjIgMCAwMC0uNTc4LjExNTRsLS40MTQxIDEuMTA2NWMtLjA5NjIuMDI3NC0uMTkwMy4wNTY3LS4yODU1LjA4Nkw3Ljc0NC45NTVhLjM0ODMuMzQ4MyAwIDAwLS41NDQ3LjIyNThMNy4wMDkgMi4zNDhhOS4zNTc0IDkuMzU3NCAwIDAwLS4yNjIyLjE0MDdsLTEuMDc2Mi0uNDkxYS4zNDYyLjM0NjIgMCAwMC0uNDkuMzI4bC4wNDE2IDEuMTg0NWE3Ljk4MjYgNy45ODI2IDAgMDAtLjIyNzguMTg3M0wzLjg0MTMgMy40MjVhLjM0NzIuMzQ3MiAwIDAwLS40MTcxLjQxNzFsLjI3MTMgMS4xNTMxYy0uMDYyOC4wNzUtLjEyNTUuMTUwOS0uMTg2My4yMjY4bC0xLjE4NDUtLjA0MTVhLjM0NjIuMzQ2MiAwIDAwLS4zMjguNDlsLjQ5MSAxLjA3NjFhOS4xNjcgOS4xNjcgMCAwMC0uMTQwNy4yNjIybC0xLjE2NjIuMTg5NGEuMzQ4My4zNDgzIDAgMDAtLjIyNTguNTQ0NmwuNjkwNC45NTg3YTEzLjMwMyAxMy4zMDMgMCAwMC0uMDg3LjI4NTVsLTEuMTA2NS40MTRhLjM0ODMuMzQ4MyAwIDAwLS4xMTU1LjU3ODFsLjg2NTYuODA3YTkuMjkzNiA5LjI5MzYgMCAwMC0uMDI4My4yOTM1bC0xLjAwNzMuNjIzNmEuMzQ0Mi4zNDQyIDAgMDAwIC41ODkybDEuMDA3My42MjM2Yy4wMDguMDk4Mi4wMTgyLjE5NjQuMDI4My4yOTM2bC0uODY1Ni44MDc5YS4zNDYyLjM0NjIgMCAwMC4xMTU1LjU3OGwxLjEwNjUuNDE0MWMuMDI3My4wOTYyLjA1NjcuMTkxNC4wODcuMjg1NWwtLjY5MDQuOTU4N2EuMzQ1Mi4zNDUyIDAgMDAuMjI2OC41NDQ3bDEuMTY2Mi4xODkzYy4wNDU2LjA4OC4wOTIyLjE3NTEuMTQwOC4yNjIybC0uNDkxIDEuMDc2MmEuMzQ2Mi4zNDYyIDAgMDAuMzI4LjQ5bDEuMTgzNC0uMDQxNWMuMDYxOC4wNzY5LjEyMzUuMTUyOC4xODczLjIyNzdsLS4yNzEzIDEuMTU0MWEuMzQ2Mi4zNDYyIDAgMDAuNDE3MS40MTYxbDEuMTUzLS4yNzEzYy4wNzUuMDYzOC4xNTEuMTI1NS4yMjc5LjE4NjNsLS4wNDE1IDEuMTg0NWEuMzQ0Mi4zNDQyIDAgMDAuNDkuMzI3bDEuMDc2MS0uNDljLjA4Ny4wNDg2LjE3NDEuMDk1MS4yNjIyLjE0MDdsLjE5MDMgMS4xNjYyYS4zNDgzLjM0ODMgMCAwMC41NDQ3LjIyNjhsLjk1ODctLjY5MDRhOS4yOTkgOS4yOTkgMCAwMC4yODU1LjA4N2wuNDE0IDEuMTA2NmEuMzQ1Mi4zNDUyIDAgMDAuNTc4MS4xMTU0bC44MDc5LS44NjU2Yy4wOTcyLjAxMTEuMTk1NC4wMjAzLjI5MzYuMDI5NGwuNjIzNiAxLjAwNzNhLjM0NzIuMzQ3MiAwIDAwLjU4OTIgMGwuNjIzNi0xLjAwNzNjLjA5ODItLjAwOTEuMTk2NC0uMDE4My4yOTM2LS4wMjk0bC44MDY5Ljg2NTZhLjM0ODMuMzQ4MyAwIDAwLjU3OC0uMTE1NGwuNDE0MS0xLjEwNjZhOC40NjI2IDguNDYyNiAwIDAwLjI4NTUtLjA4N2wuOTU4Ny42OTA0YS4zNDUyLjM0NTIgMCAwMC41NDQ3LS4yMjY4bC4xOTAzLTEuMTY2MmMuMDg4LS4wNDU2LjE3NTEtLjA5MzEuMjYyMi0uMTQwN2wxLjA3NjIuNDlhLjM0NzIuMzQ3MiAwIDAwLjQ5LS4zMjdsLS4wNDE1LTEuMTg0NWE2LjcyNjcgNi43MjY3IDAgMDAuMjI2Ny0uMTg2M2wxLjE1MzEuMjcxM2EuMzQ3Mi4zNDcyIDAgMDAuNDE3MS0uNDE2bC0uMjcxMy0xLjE1NDJjLjA2MjgtLjA3NDkuMTI1NS0uMTUwOC4xODYzLS4yMjc4bDEuMTg0NS4wNDE1YS4zNDQyLjM0NDIgMCAwMC4zMjgtLjQ5bC0uNDktMS4wNzZjLjA0NzUtLjA4NzIuMDk1MS0uMTc0Mi4xNDA3LS4yNjIzbDEuMTY2Mi0uMTg5M2EuMzQ4My4zNDgzIDAgMDAuMjI1OC0uNTQ0N2wtLjY5MDQtLjk1ODcuMDg3LS4yODU1IDEuMTA2Ni0uNDE0YS4zNDYyLjM0NjIgMCAwMC4xMTU0LS41NzgxbC0uODY1Ni0uODA3OWMuMDEwMS0uMDk3Mi4wMjAyLS4xOTU0LjAyODMtLjI5MzZsMS4wMDczLS42MjM2YS4zNDQyLjM0NDIgMCAwMDAtLjU4OTJ6bS02Ljc0MTMgOC4zNTUxYS43MTM4LjcxMzggMCAwMS4yOTg2LTEuMzk2LjcxNC43MTQgMCAxMS0uMjk5NyAxLjM5NnptLS4zNDIyLTIuMzE0MmEuNjQ5LjY0OSAwIDAwLS43NzE1LjVsLS4zNTczIDEuNjY4NWMtMS4xMDM1LjUwMS0yLjMyODUuNzc5NS0zLjYxOTMuNzc5NWE4LjczNjggOC43MzY4IDAgMDEtMy42OTUxLS44MTRsLS4zNTc0LTEuNjY4NGEuNjQ4LjY0OCAwIDAwLS43NzE0LS40OTlsLTEuNDczLjMxNThhOC43MjE2IDguNzIxNiAwIDAxLS43NjEzLS44OThoNy4xNjc2Yy4wODEgMCAuMTM1Ni0uMDE0MS4xMzU2LS4wODh2LTIuNTM2YzAtLjA3NC0uMDUzNi0uMDg4MS0uMTM1Ni0uMDg4MWgtMi4wOTY2di0xLjYwNzdoMi4yNjc3Yy4yMDY1IDAgMS4xMDY1LjA1ODcgMS4zOTQgMS4yMDg4LjA5MDEuMzUzMy4yODc1IDEuNTA0NC40MjMyIDEuODcyOS4xMzQ2LjQxMy42ODMzIDEuMjM4MSAxLjI2ODUgMS4yMzgxaDMuNTcxNmEuNzQ5Mi43NDkyIDAgMDAuMTI5Ni0uMDEzMSA4Ljc4NzQgOC43ODc0IDAgMDEtLjgxMTkuOTUyNnpNNi44MzY5IDIwLjAyNGEuNzE0LjcxNCAwIDExLS4yOTk3LTEuMzk2LjcxNC43MTQgMCAwMS4yOTk3IDEuMzk2ek00LjExNzcgOC45OTcyYS43MTM3LjcxMzcgMCAxMS0xLjMwNC41NzkxLjcxMzcuNzEzNyAwIDAxMS4zMDQtLjU3OXptLS44MzUyIDEuOTgxM2wxLjUzNDctLjY4MjRhLjY1LjY1IDAgMDAuMzMtLjg1ODVsLS4zMTU4LS43MTQ3aDEuMjQzMnY1LjYwMjVIMy41NjY5YTguNzc1MyA4Ljc3NTMgMCAwMS0uMjgzNC0zLjM0OHptNi43MzQzLS41NDM3VjguNzgzNmgyLjk2MDFjLjE1MyAwIDEuMDc5Mi4xNzcyIDEuMDc5Mi44Njk3IDAgLjU3NS0uNzEwNy43ODE1LTEuMjk0OC43ODE1em0xMC43NTc0IDEuNDg2MmMwIC4yMTg3LS4wMDguNDM2My0uMDI0My42NTFoLS45Yy0uMDkgMC0uMTI2NS4wNTg2LS4xMjY1LjE0Nzd2LjQxM2MwIC45NzMtLjU0ODcgMS4xODQ2LTEuMDI5NiAxLjIzODItLjQ1NzYuMDUxNy0uOTY0OC0uMTkxMy0xLjAyNzUtLjQ3MTctLjI3MDQtMS41MTg2LS43MTk4LTEuODQzNi0xLjQzMDUtMi40MDM0Ljg4MTctLjU1OTkgMS43OTktMS4zODYgMS43OTktMi40OTE1IDAtMS4xOTM2LS44MTktMS45NDU4LTEuMzc2OS0yLjMxNTMtLjc4MjUtLjUxNjMtMS42NDkxLS42MTk1LTEuODgzLS42MTk1SDUuNDY4MmE4Ljc2NTEgOC43NjUxIDAgMDE0LjkwNy0yLjc2OTlsMS4wOTc0IDEuMTUxYS42NDguNjQ4IDAgMDAuOTE4Mi4wMjEzbDEuMjI3LTEuMTc0M2E4Ljc3NTMgOC43NzUzIDAgMDE2LjAwNDQgNC4yNzYybC0uODQwMyAxLjg5ODJhLjY1Mi42NTIgMCAwMC4zMy44NTg1bDEuNjE3OC43MTg4Yy4wMjgzLjI4NzUuMDQyNS41NzcuMDQyNS44NzE3em0tOS4zMDA2LTkuNTk5M2EuNzEyOC43MTI4IDAgMTEuOTg0IDEuMDMxNi43MTM3LjcxMzcgMCAwMS0uOTg0LTEuMDMxNnptOC4zMzg5IDYuNzFhLjcxMDcuNzEwNyAwIDAxLjkzOTUtLjM2MjUuNzEzNy43MTM3IDAgMTEtLjk0MDUuMzYzNXoiLz48L3N2Zz4="/>        <g aria-hidden="true" fill="#333" text-anchor="middle" font-family="Helvetica Neue,Helvetica,Arial,sans-serif" text-rendering="geometricPrecision" font-weight="700" font-size="110px" line-height="14px">        <rect id="llink" stroke="#d5d5d5" fill="url(#smoothzQkNqHB)" x=".5" y=".5" width="52.226604" height="19" rx="2" />        <text aria-hidden="true" x="346.13306" y="150" fill="#fff" transform="scale(.1)" textLength="252.26605">Rust</text>        <text x="346.13306" y="140" transform="scale(.1)" textLength="252.26605">Rust</text>        <text aria-hidden="true" x="1068.8777" y="150" fill="#fff" transform="scale(.1)" textLength="893.2234">so hot right now</text>        <text id="rlink" x="1068.8777" y="140" transform="scale(.1)" textLength="893.2234">so hot right now</text>    </g>                </svg>
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

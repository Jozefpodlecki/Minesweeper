(window["webpackJsonp"] = window["webpackJsonp"] || []).push([["main"],{

/***/ "./$$_lazy_route_resource lazy recursive":
/*!******************************************************!*\
  !*** ./$$_lazy_route_resource lazy namespace object ***!
  \******************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

function webpackEmptyAsyncContext(req) {
	// Here Promise.resolve().then() is used instead of new Promise() to prevent
	// uncaught exception popping up in devtools
	return Promise.resolve().then(function() {
		var e = new Error("Cannot find module '" + req + "'");
		e.code = 'MODULE_NOT_FOUND';
		throw e;
	});
}
webpackEmptyAsyncContext.keys = function() { return []; };
webpackEmptyAsyncContext.resolve = webpackEmptyAsyncContext;
module.exports = webpackEmptyAsyncContext;
webpackEmptyAsyncContext.id = "./$$_lazy_route_resource lazy recursive";

/***/ }),

/***/ "./src/app/app-routing.module.ts":
/*!***************************************!*\
  !*** ./src/app/app-routing.module.ts ***!
  \***************************************/
/*! exports provided: AppRoutingModule */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "AppRoutingModule", function() { return AppRoutingModule; });
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! @angular/core */ "./node_modules/@angular/core/__ivy_ngcc__/fesm2015/core.js");
/* harmony import */ var _angular_router__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/router */ "./node_modules/@angular/router/__ivy_ngcc__/fesm2015/router.js");
/* harmony import */ var _game_game_component__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./game/game.component */ "./src/app/game/game.component.ts");





const routes = [{
        path: '',
        component: _game_game_component__WEBPACK_IMPORTED_MODULE_2__["GameComponent"]
    }];
class AppRoutingModule {
}
AppRoutingModule.ɵmod = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵdefineNgModule"]({ type: AppRoutingModule });
AppRoutingModule.ɵinj = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵdefineInjector"]({ factory: function AppRoutingModule_Factory(t) { return new (t || AppRoutingModule)(); }, imports: [[_angular_router__WEBPACK_IMPORTED_MODULE_1__["RouterModule"].forRoot(routes)],
        _angular_router__WEBPACK_IMPORTED_MODULE_1__["RouterModule"]] });
(function () { (typeof ngJitMode === "undefined" || ngJitMode) && _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵsetNgModuleScope"](AppRoutingModule, { imports: [_angular_router__WEBPACK_IMPORTED_MODULE_1__["RouterModule"]], exports: [_angular_router__WEBPACK_IMPORTED_MODULE_1__["RouterModule"]] }); })();
/*@__PURE__*/ (function () { _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵsetClassMetadata"](AppRoutingModule, [{
        type: _angular_core__WEBPACK_IMPORTED_MODULE_0__["NgModule"],
        args: [{
                imports: [_angular_router__WEBPACK_IMPORTED_MODULE_1__["RouterModule"].forRoot(routes)],
                exports: [_angular_router__WEBPACK_IMPORTED_MODULE_1__["RouterModule"]]
            }]
    }], null, null); })();


/***/ }),

/***/ "./src/app/app.component.ts":
/*!**********************************!*\
  !*** ./src/app/app.component.ts ***!
  \**********************************/
/*! exports provided: AppComponent */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "AppComponent", function() { return AppComponent; });
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! @angular/core */ "./node_modules/@angular/core/__ivy_ngcc__/fesm2015/core.js");
/* harmony import */ var _angular_router__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/router */ "./node_modules/@angular/router/__ivy_ngcc__/fesm2015/router.js");



class AppComponent {
    constructor() {
        this.signature = `Józef Podlecki ${new Date().getFullYear()}`;
    }
}
AppComponent.ɵfac = function AppComponent_Factory(t) { return new (t || AppComponent)(); };
AppComponent.ɵcmp = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵdefineComponent"]({ type: AppComponent, selectors: [["app-root"]], decls: 7, vars: 1, consts: [[1, "background"], [1, "content"], [1, "footer"], [1, "footer__signature"], [1, "footer__social"]], template: function AppComponent_Template(rf, ctx) { if (rf & 1) {
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](0, "div", 0);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](1, "div", 1);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](2, "router-outlet");
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](3, "footer", 2);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](4, "div", 3);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtext"](5);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](6, "div", 4);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
    } if (rf & 2) {
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](5);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtextInterpolate"](ctx.signature);
    } }, directives: [_angular_router__WEBPACK_IMPORTED_MODULE_1__["RouterOutlet"]], styles: ["[_nghost-%COMP%] {\n  display: flex;\n  flex-direction: column;\n  height: inherit;\n}\n.background[_ngcontent-%COMP%] {\n  background: url(/assets/background.png) center center / cover no-repeat;\n  position: absolute;\n  width: 100%;\n  height: 100%;\n  z-index: -1;\n  -webkit-filter: brightness(0.8);\n          filter: brightness(0.8);\n}\n.content[_ngcontent-%COMP%] {\n  flex: 1 1 auto;\n}\n.footer[_ngcontent-%COMP%] {\n  flex: 0 0 auto;\n  padding: 1rem;\n}\n/*# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbInNyYy9hcHAvQzovVXNlcnMvSsOzemVmIFBvZGxlY2tpL0RvY3VtZW50cy9yZXBvcy9NaW5lc3dlZXBlci9zcmMvYXBwL2FwcC5jb21wb25lbnQubGVzcyIsInNyYy9hcHAvYXBwLmNvbXBvbmVudC5sZXNzIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQUFBO0VBQ0ksYUFBQTtFQUNBLHNCQUFBO0VBQ0EsZUFBQTtBQ0NKO0FERUE7RUFDSSx1RUFBQTtFQUNBLGtCQUFBO0VBQ0EsV0FBQTtFQUNBLFlBQUE7RUFDQSxXQUFBO0VBQ0EsK0JBQUE7VUFBQSx1QkFBQTtBQ0FKO0FER0E7RUFDSSxjQUFBO0FDREo7QURJQTtFQUNJLGNBQUE7RUFDQSxhQUFBO0FDRkoiLCJmaWxlIjoic3JjL2FwcC9hcHAuY29tcG9uZW50Lmxlc3MiLCJzb3VyY2VzQ29udGVudCI6WyI6aG9zdCB7XG4gICAgZGlzcGxheTogZmxleDtcbiAgICBmbGV4LWRpcmVjdGlvbjogY29sdW1uO1xuICAgIGhlaWdodDogaW5oZXJpdDtcbn1cblxuLmJhY2tncm91bmQge1xuICAgIGJhY2tncm91bmQ6IHVybCgvYXNzZXRzL2JhY2tncm91bmQucG5nKSBjZW50ZXIgY2VudGVyIC8gY292ZXIgbm8tcmVwZWF0O1xuICAgIHBvc2l0aW9uOiBhYnNvbHV0ZTtcbiAgICB3aWR0aDogMTAwJTtcbiAgICBoZWlnaHQ6IDEwMCU7XG4gICAgei1pbmRleDogLTE7XG4gICAgZmlsdGVyOiBicmlnaHRuZXNzKC44KTtcbn1cblxuLmNvbnRlbnQge1xuICAgIGZsZXg6IDEgMSBhdXRvO1xufVxuXG4uZm9vdGVyIHtcbiAgICBmbGV4OiAwIDAgYXV0bztcbiAgICBwYWRkaW5nOiAxcmVtO1xufSIsIjpob3N0IHtcbiAgZGlzcGxheTogZmxleDtcbiAgZmxleC1kaXJlY3Rpb246IGNvbHVtbjtcbiAgaGVpZ2h0OiBpbmhlcml0O1xufVxuLmJhY2tncm91bmQge1xuICBiYWNrZ3JvdW5kOiB1cmwoL2Fzc2V0cy9iYWNrZ3JvdW5kLnBuZykgY2VudGVyIGNlbnRlciAvIGNvdmVyIG5vLXJlcGVhdDtcbiAgcG9zaXRpb246IGFic29sdXRlO1xuICB3aWR0aDogMTAwJTtcbiAgaGVpZ2h0OiAxMDAlO1xuICB6LWluZGV4OiAtMTtcbiAgZmlsdGVyOiBicmlnaHRuZXNzKDAuOCk7XG59XG4uY29udGVudCB7XG4gIGZsZXg6IDEgMSBhdXRvO1xufVxuLmZvb3RlciB7XG4gIGZsZXg6IDAgMCBhdXRvO1xuICBwYWRkaW5nOiAxcmVtO1xufVxuIl19 */"] });
/*@__PURE__*/ (function () { _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵsetClassMetadata"](AppComponent, [{
        type: _angular_core__WEBPACK_IMPORTED_MODULE_0__["Component"],
        args: [{
                selector: 'app-root',
                templateUrl: './app.component.html',
                styleUrls: ['./app.component.less']
            }]
    }], function () { return []; }, null); })();


/***/ }),

/***/ "./src/app/app.module.ts":
/*!*******************************!*\
  !*** ./src/app/app.module.ts ***!
  \*******************************/
/*! exports provided: AppModule */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "AppModule", function() { return AppModule; });
/* harmony import */ var _angular_platform_browser__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! @angular/platform-browser */ "./node_modules/@angular/platform-browser/__ivy_ngcc__/fesm2015/platform-browser.js");
/* harmony import */ var _angular_platform_browser_animations__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/platform-browser/animations */ "./node_modules/@angular/platform-browser/__ivy_ngcc__/fesm2015/animations.js");
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! @angular/core */ "./node_modules/@angular/core/__ivy_ngcc__/fesm2015/core.js");
/* harmony import */ var _fortawesome_angular_fontawesome__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! @fortawesome/angular-fontawesome */ "./node_modules/@fortawesome/angular-fontawesome/__ivy_ngcc__/fesm2015/angular-fontawesome.js");
/* harmony import */ var _app_routing_module__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./app-routing.module */ "./src/app/app-routing.module.ts");
/* harmony import */ var _app_component__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./app.component */ "./src/app/app.component.ts");
/* harmony import */ var _game_game_component__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! ./game/game.component */ "./src/app/game/game.component.ts");
/* harmony import */ var _minesweeper_service_minesweeper_service__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! ../minesweeper-service/minesweeper.service */ "./src/minesweeper-service/minesweeper.service.ts");









class AppModule {
}
AppModule.ɵmod = _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵdefineNgModule"]({ type: AppModule, bootstrap: [_app_component__WEBPACK_IMPORTED_MODULE_5__["AppComponent"]] });
AppModule.ɵinj = _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵdefineInjector"]({ factory: function AppModule_Factory(t) { return new (t || AppModule)(); }, providers: [_minesweeper_service_minesweeper_service__WEBPACK_IMPORTED_MODULE_7__["MinesweeperService"]], imports: [[
            _angular_platform_browser__WEBPACK_IMPORTED_MODULE_0__["BrowserModule"],
            _app_routing_module__WEBPACK_IMPORTED_MODULE_4__["AppRoutingModule"],
            _fortawesome_angular_fontawesome__WEBPACK_IMPORTED_MODULE_3__["FontAwesomeModule"],
            _angular_platform_browser_animations__WEBPACK_IMPORTED_MODULE_1__["BrowserAnimationsModule"]
        ]] });
(function () { (typeof ngJitMode === "undefined" || ngJitMode) && _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵsetNgModuleScope"](AppModule, { declarations: [_app_component__WEBPACK_IMPORTED_MODULE_5__["AppComponent"],
        _game_game_component__WEBPACK_IMPORTED_MODULE_6__["GameComponent"]], imports: [_angular_platform_browser__WEBPACK_IMPORTED_MODULE_0__["BrowserModule"],
        _app_routing_module__WEBPACK_IMPORTED_MODULE_4__["AppRoutingModule"],
        _fortawesome_angular_fontawesome__WEBPACK_IMPORTED_MODULE_3__["FontAwesomeModule"],
        _angular_platform_browser_animations__WEBPACK_IMPORTED_MODULE_1__["BrowserAnimationsModule"]] }); })();
/*@__PURE__*/ (function () { _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵsetClassMetadata"](AppModule, [{
        type: _angular_core__WEBPACK_IMPORTED_MODULE_2__["NgModule"],
        args: [{
                declarations: [
                    _app_component__WEBPACK_IMPORTED_MODULE_5__["AppComponent"],
                    _game_game_component__WEBPACK_IMPORTED_MODULE_6__["GameComponent"]
                ],
                imports: [
                    _angular_platform_browser__WEBPACK_IMPORTED_MODULE_0__["BrowserModule"],
                    _app_routing_module__WEBPACK_IMPORTED_MODULE_4__["AppRoutingModule"],
                    _fortawesome_angular_fontawesome__WEBPACK_IMPORTED_MODULE_3__["FontAwesomeModule"],
                    _angular_platform_browser_animations__WEBPACK_IMPORTED_MODULE_1__["BrowserAnimationsModule"]
                ],
                providers: [_minesweeper_service_minesweeper_service__WEBPACK_IMPORTED_MODULE_7__["MinesweeperService"]],
                bootstrap: [_app_component__WEBPACK_IMPORTED_MODULE_5__["AppComponent"]]
            }]
    }], null, null); })();


/***/ }),

/***/ "./src/app/game/game.component.ts":
/*!****************************************!*\
  !*** ./src/app/game/game.component.ts ***!
  \****************************************/
/*! exports provided: GameComponent */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "GameComponent", function() { return GameComponent; });
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! @angular/core */ "./node_modules/@angular/core/__ivy_ngcc__/fesm2015/core.js");
/* harmony import */ var _fortawesome_free_solid_svg_icons__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @fortawesome/free-solid-svg-icons */ "./node_modules/@fortawesome/free-solid-svg-icons/index.es.js");
/* harmony import */ var _angular_animations__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! @angular/animations */ "./node_modules/@angular/animations/__ivy_ngcc__/fesm2015/animations.js");
/* harmony import */ var src_minesweeper_service_minesweeper_service__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! src/minesweeper-service/minesweeper.service */ "./src/minesweeper-service/minesweeper.service.ts");
/* harmony import */ var _angular_common__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/common */ "./node_modules/@angular/common/__ivy_ngcc__/fesm2015/common.js");
/* harmony import */ var _fortawesome_angular_fontawesome__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! @fortawesome/angular-fontawesome */ "./node_modules/@fortawesome/angular-fontawesome/__ivy_ngcc__/fesm2015/angular-fontawesome.js");







function GameComponent_div_3_Template(rf, ctx) { if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](0, "div", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](1, "div");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtext"](2, "Loading");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](3, "div");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](4, "fa-icon", 9);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](5, "fa-icon", 9);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](6, "fa-icon", 9);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
} if (rf & 2) {
    const ctx_r0 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵnextContext"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("@enterTrigger", "loading");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](4);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("icon", ctx_r0.faBomb);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("icon", ctx_r0.faBomb);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("icon", ctx_r0.faBomb);
} }
function GameComponent_div_4_div_1_div_1_Template(rf, ctx) { if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](0, "div", 17);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtext"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
} if (rf & 2) {
    const cell_r5 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵnextContext"]().$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("@enterTrigger", "number");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtextInterpolate"](cell_r5.bombCount);
} }
function GameComponent_div_4_div_1_fa_icon_2_Template(rf, ctx) { if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](0, "fa-icon", 9);
} if (rf & 2) {
    const ctx_r7 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵnextContext"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("icon", ctx_r7.faBomb)("@enterTrigger", "bomb");
} }
function GameComponent_div_4_div_1_fa_icon_3_Template(rf, ctx) { if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](0, "fa-icon", 9);
} if (rf & 2) {
    const ctx_r8 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵnextContext"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("icon", ctx_r8.faFlag);
} }
function GameComponent_div_4_div_1_div_4_Template(rf, ctx) { if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](0, "div", 18);
} if (rf & 2) {
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("@enterTrigger", "revealed");
} }
function GameComponent_div_4_div_1_Template(rf, ctx) { if (rf & 1) {
    const _r12 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵgetCurrentView"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](0, "div", 13);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵlistener"]("click", function GameComponent_div_4_div_1_Template_div_click_0_listener() { _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵrestoreView"](_r12); const cell_r5 = ctx.$implicit; const ctx_r11 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵnextContext"](2); return ctx_r11.click(cell_r5); })("contextmenu", function GameComponent_div_4_div_1_Template_div_contextmenu_0_listener($event) { _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵrestoreView"](_r12); const cell_r5 = ctx.$implicit; const ctx_r13 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵnextContext"](2); return ctx_r13.onRightClick(cell_r5, $event); });
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtemplate"](1, GameComponent_div_4_div_1_div_1_Template, 2, 2, "div", 14);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtemplate"](2, GameComponent_div_4_div_1_fa_icon_2_Template, 1, 2, "fa-icon", 15);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtemplate"](3, GameComponent_div_4_div_1_fa_icon_3_Template, 1, 1, "fa-icon", 15);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtemplate"](4, GameComponent_div_4_div_1_div_4_Template, 1, 1, "div", 16);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
} if (rf & 2) {
    const cell_r5 = ctx.$implicit;
    const ctx_r4 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵnextContext"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("ngSwitch", ctx_r4.computeState(cell_r5));
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("ngSwitchCase", "count");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("ngSwitchCase", "bomb");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("ngSwitchCase", "flag");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("ngSwitchCase", "revealed");
} }
function GameComponent_div_4_Template(rf, ctx) { if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](0, "div", 11);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtemplate"](1, GameComponent_div_4_div_1_Template, 5, 5, "div", 12);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
} if (rf & 2) {
    const ctx_r1 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵnextContext"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵclassProp"]("game__grid--disabled", ctx_r1.disabled);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("@enterTrigger", "game");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("ngForOf", ctx_r1.board);
} }
function GameComponent_div_5_Template(rf, ctx) { if (rf & 1) {
    const _r15 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵgetCurrentView"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](0, "div", 19);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵlistener"]("contextmenu", function GameComponent_div_5_Template_div_contextmenu_0_listener($event) { _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵrestoreView"](_r15); return $event.preventDefault(); });
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](1, "div", 20);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](2, "span", 21);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtext"](3, "You won");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](4, "fa-icon", 22);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](5, "div", 23);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵlistener"]("contextmenu", function GameComponent_div_5_Template_div_contextmenu_5_listener() { _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵrestoreView"](_r15); const ctx_r16 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵnextContext"](); return ctx_r16.initialize(); });
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](6, "span", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtext"](7, "Play again");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](8, "fa-icon", 9);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
} if (rf & 2) {
    const ctx_r2 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵnextContext"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("@enterTrigger", "won");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](4);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("icon", ctx_r2.faSmile);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](4);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("icon", ctx_r2.faBomb);
} }
function GameComponent_div_6_Template(rf, ctx) { if (rf & 1) {
    const _r18 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵgetCurrentView"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](0, "div", 25);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵlistener"]("click", function GameComponent_div_6_Template_div_click_0_listener($event) { _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵrestoreView"](_r18); return $event.preventDefault(); });
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](1, "div", 20);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](2, "div", 21);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtext"](3, "You lost");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](4, "fa-icon", 22);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](5, "div", 26);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵlistener"]("click", function GameComponent_div_6_Template_div_click_5_listener() { _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵrestoreView"](_r18); const ctx_r19 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵnextContext"](); return ctx_r19.initialize(); });
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](6, "span", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtext"](7, "Play again");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](8, "fa-icon", 9);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
} if (rf & 2) {
    const ctx_r3 = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵnextContext"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("@enterTrigger", "lost");
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](4);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("icon", ctx_r3.faFrown);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](4);
    _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("icon", ctx_r3.faBomb);
} }
var GameState;
(function (GameState) {
    GameState[GameState["game"] = 0] = "game";
    GameState[GameState["won"] = 1] = "won";
    GameState[GameState["lost"] = 2] = "lost";
    GameState[GameState["loading"] = 3] = "loading";
})(GameState || (GameState = {}));
class GameComponent {
    constructor(_minesweeperService) {
        this._minesweeperService = _minesweeperService;
        this.faBomb = _fortawesome_free_solid_svg_icons__WEBPACK_IMPORTED_MODULE_1__["faBomb"];
        this.faFlag = _fortawesome_free_solid_svg_icons__WEBPACK_IMPORTED_MODULE_1__["faFlag"];
        this.faSyncAlt = _fortawesome_free_solid_svg_icons__WEBPACK_IMPORTED_MODULE_1__["faSyncAlt"];
        this.faSmile = _fortawesome_free_solid_svg_icons__WEBPACK_IMPORTED_MODULE_1__["faSmile"];
        this.faFrown = _fortawesome_free_solid_svg_icons__WEBPACK_IMPORTED_MODULE_1__["faFrown"];
        this.GameState = GameState;
        this.initialize();
    }
    initialize() {
        this.disabled = true;
        this.state = GameState.loading;
        this._minesweeperService.initializeBoard().then(board => {
            this.board = board;
            this.state = GameState.game;
            this.disabled = false;
        });
    }
    ngOnInit() {
    }
    computeState(cell) {
        if (cell.isRevealed) {
            if (cell.hasBomb) {
                return 'bomb';
            }
            if (cell.bombCount) {
                return 'count';
            }
            return 'revealed';
        }
        if (cell.isFlagged) {
            return 'flag';
        }
        return 'unrevealed';
    }
    click(cell) {
        cell.isRevealed = true;
        if (cell.hasBomb) {
            this.gameOver();
            return;
        }
        if (!cell.bombCount) {
            const cells = this._minesweeperService.uncoverNeighbors(this.board, cell);
            for (let cell of cells) {
                cell.isRevealed = true;
            }
        }
        this.checkIfGameOver();
    }
    checkIfGameOver() {
        const allRevealed = this.board.some(({ isRevealed }) => isRevealed === true);
        const allFlagged = !this.board.some(({ hasBomb, isFlagged }) => hasBomb !== isFlagged);
        if (allFlagged && allRevealed) {
            this.state = GameState.won;
        }
    }
    onRightClick(cell, event) {
        cell.isFlagged = !cell.isFlagged;
        this.checkIfGameOver();
        event.preventDefault();
    }
    gameOver() {
        this.disabled = true;
        for (let cell of this.board) {
            cell.isRevealed = true;
        }
        setTimeout(() => {
            this.state = GameState.lost;
        }, 1000);
    }
}
GameComponent.ɵfac = function GameComponent_Factory(t) { return new (t || GameComponent)(_angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵdirectiveInject"](src_minesweeper_service_minesweeper_service__WEBPACK_IMPORTED_MODULE_3__["MinesweeperService"])); };
GameComponent.ɵcmp = _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵdefineComponent"]({ type: GameComponent, selectors: [["game"]], decls: 12, vars: 8, consts: [[1, "game__header"], [1, "game", 3, "ngSwitch"], ["class", "game__loading", 4, "ngSwitchCase"], ["class", "game__grid", 3, "game__grid--disabled", 4, "ngSwitchCase"], ["class", "game__statusWrapper", 3, "contextmenu", 4, "ngSwitchCase"], ["class", "game__statusWrapper", 3, "click", 4, "ngSwitchCase"], [1, "actions"], [1, "actions__give-up", 3, "click"], [1, "actions__reset", 3, "click"], ["size", "2x", 3, "icon"], [1, "game__loading"], [1, "game__grid"], ["class", "game__cell", 3, "ngSwitch", "click", "contextmenu", 4, "ngFor", "ngForOf"], [1, "game__cell", 3, "ngSwitch", "click", "contextmenu"], ["class", "number", 4, "ngSwitchCase"], ["size", "2x", 3, "icon", 4, "ngSwitchCase"], ["class", "revealed", 4, "ngSwitchCase"], [1, "number"], [1, "revealed"], [1, "game__statusWrapper", 3, "contextmenu"], [1, "game__status"], [1, "game__statusHeader"], ["size", "4x", 3, "icon"], [1, "game__playAgain", 3, "contextmenu"], [1, "game__playAgainText"], [1, "game__statusWrapper", 3, "click"], [1, "game__playAgain", 3, "click"]], template: function GameComponent_Template(rf, ctx) { if (rf & 1) {
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](0, "div", 0);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtext"](1, "Minesweeper");
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](2, "div", 1);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtemplate"](3, GameComponent_div_3_Template, 7, 4, "div", 2);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtemplate"](4, GameComponent_div_4_Template, 2, 4, "div", 3);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtemplate"](5, GameComponent_div_5_Template, 9, 3, "div", 4);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtemplate"](6, GameComponent_div_6_Template, 9, 3, "div", 5);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](7, "div", 6);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](8, "div", 7);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵlistener"]("click", function GameComponent_Template_div_click_8_listener() { return ctx.gameOver(); });
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵtext"](9, "I give up");
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementStart"](10, "div", 8);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵlistener"]("click", function GameComponent_Template_div_click_10_listener() { return ctx.initialize(); });
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelement"](11, "fa-icon", 9);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵelementEnd"]();
    } if (rf & 2) {
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](2);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("ngSwitch", ctx.state);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](1);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("ngSwitchCase", ctx.GameState.loading);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](1);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("ngSwitchCase", ctx.GameState.game);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](1);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("ngSwitchCase", ctx.GameState.won);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](1);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("ngSwitchCase", ctx.GameState.lost);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](2);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵclassProp"]("actions__give-up--disabled", ctx.state !== ctx.GameState.game);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵadvance"](3);
        _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵproperty"]("icon", ctx.faSyncAlt);
    } }, directives: [_angular_common__WEBPACK_IMPORTED_MODULE_4__["NgSwitch"], _angular_common__WEBPACK_IMPORTED_MODULE_4__["NgSwitchCase"], _fortawesome_angular_fontawesome__WEBPACK_IMPORTED_MODULE_5__["FaIconComponent"], _angular_common__WEBPACK_IMPORTED_MODULE_4__["NgForOf"]], styles: ["[_nghost-%COMP%] {\n  display: block;\n  height: 100%;\n}\n.game[_ngcontent-%COMP%] {\n  display: flex;\n  justify-content: center;\n}\n.game__header[_ngcontent-%COMP%] {\n  font-size: 4rem;\n  padding: 1rem 0;\n  text-align: center;\n}\n.game__grid[_ngcontent-%COMP%] {\n  display: grid;\n  grid-template-columns: repeat(10, 50px);\n  grid-template-rows: repeat(10, 50px);\n  cursor: pointer;\n}\n.game__grid--disabled[_ngcontent-%COMP%] {\n  cursor: not-allowed;\n  pointer-events: none;\n}\n.game__statusWrapper[_ngcontent-%COMP%] {\n  display: flex;\n  flex-direction: column;\n  justify-content: center;\n  height: 500px;\n}\n.game__playAgain[_ngcontent-%COMP%], .game__status[_ngcontent-%COMP%] {\n  display: flex;\n  justify-content: center;\n  align-items: center;\n}\n.game__statusHeader[_ngcontent-%COMP%] {\n  font-size: 3rem;\n  margin-right: 1rem;\n}\n.game__playAgain[_ngcontent-%COMP%] {\n  cursor: pointer;\n  margin-top: 2rem;\n  padding: 0.5rem 0;\n  transition: border 0.3s ease-in;\n  border: 1px solid transparent;\n}\n.game__playAgain[_ngcontent-%COMP%]:hover {\n  border: 1px solid;\n}\n.game__playAgainText[_ngcontent-%COMP%] {\n  margin-right: 1rem;\n}\n.game__cell[_ngcontent-%COMP%] {\n  display: flex;\n  justify-content: center;\n  align-items: center;\n  transition: all 0.3s ease-in;\n  border: 1px solid #FFFFFF99;\n}\n.number[_ngcontent-%COMP%] {\n  font-size: 2rem;\n}\n.revealed[_ngcontent-%COMP%] {\n  background: white;\n  width: 100%;\n  height: 100%;\n}\n.actions[_ngcontent-%COMP%] {\n  display: flex;\n  align-items: center;\n  justify-content: space-around;\n  padding: 1rem 0;\n  width: 500px;\n  margin: auto;\n}\n.actions__reset[_ngcontent-%COMP%], .actions__give-up[_ngcontent-%COMP%] {\n  cursor: pointer;\n}\n.actions__give-up[_ngcontent-%COMP%] {\n  font-size: 2rem;\n  padding: 0.5rem;\n  border: 1px solid #FFFFFF99;\n}\n.actions__give-up--disabled[_ngcontent-%COMP%] {\n  color: gray;\n  border-color: gray;\n  pointer-events: none;\n  cursor: none;\n}\n/*# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbInNyYy9hcHAvZ2FtZS9DOi9Vc2Vycy9Kw7N6ZWYgUG9kbGVja2kvRG9jdW1lbnRzL3JlcG9zL01pbmVzd2VlcGVyL3NyYy9hcHAvZ2FtZS9nYW1lLmNvbXBvbmVudC5sZXNzIiwic3JjL2FwcC9nYW1lL2dhbWUuY29tcG9uZW50Lmxlc3MiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBQUE7RUFDSSxjQUFBO0VBQ0EsWUFBQTtBQ0NKO0FESUE7RUFDSSxhQUFBO0VBQ0EsdUJBQUE7QUNGSjtBRElJO0VBQ0ksZUFBQTtFQUNBLGVBQUE7RUFDQSxrQkFBQTtBQ0ZSO0FES0k7RUFDSSxhQUFBO0VBQ0EsdUNBQUE7RUFDQSxvQ0FBQTtFQUNBLGVBQUE7QUNIUjtBREtRO0VBQ0ksbUJBQUE7RUFDQSxvQkFBQTtBQ0haO0FET0k7RUFDSSxhQUFBO0VBQ0Esc0JBQUE7RUFDQSx1QkFBQTtFQUNBLGFBQUE7QUNMUjtBRFFJOztFQUNJLGFBQUE7RUFDQSx1QkFBQTtFQUNBLG1CQUFBO0FDTFI7QURRSTtFQUNJLGVBQUE7RUFDQSxrQkFBQTtBQ05SO0FEU0k7RUFDSSxlQUFBO0VBQ0EsZ0JBQUE7RUFDQSxpQkFBQTtFQUNBLCtCQUFBO0VBQ0EsNkJBQUE7QUNQUjtBRFNRO0VBQ0ksaUJBQUE7QUNQWjtBRFdJO0VBQ0ksa0JBQUE7QUNUUjtBRFlJO0VBQ0ksYUFBQTtFQUNBLHVCQUFBO0VBQ0EsbUJBQUE7RUFDQSw0QkFBQTtFQUNBLDJCQUFBO0FDVlI7QURjQTtFQUNJLGVBQUE7QUNaSjtBRGVBO0VBQ0ksaUJBQUE7RUFDQSxXQUFBO0VBQ0EsWUFBQTtBQ2JKO0FEb0JBO0VBQ0ksYUFBQTtFQUNBLG1CQUFBO0VBQ0EsNkJBQUE7RUFDQSxlQUFBO0VBQ0EsWUFBQTtFQUNBLFlBQUE7QUNsQko7QURvQkk7O0VBQ0ksZUFBQTtBQ2pCUjtBRG9CSTtFQUNJLGVBQUE7RUFDQSxlQUFBO0VBQ0EsMkJBQUE7QUNsQlI7QURvQlE7RUFDSSxXQUFBO0VBQ0Esa0JBQUE7RUFDQSxvQkFBQTtFQUNBLFlBQUE7QUNsQloiLCJmaWxlIjoic3JjL2FwcC9nYW1lL2dhbWUuY29tcG9uZW50Lmxlc3MiLCJzb3VyY2VzQ29udGVudCI6WyI6aG9zdCB7XG4gICAgZGlzcGxheTogYmxvY2s7XG4gICAgaGVpZ2h0OiAxMDAlO1xufVxuXG5Ad2hpdGU6ICNGRkZGRkY5OTtcblxuLmdhbWUge1xuICAgIGRpc3BsYXk6IGZsZXg7XG4gICAganVzdGlmeS1jb250ZW50OiBjZW50ZXI7XG5cbiAgICAmX19oZWFkZXIge1xuICAgICAgICBmb250LXNpemU6IDRyZW07XG4gICAgICAgIHBhZGRpbmc6IDFyZW0gMDtcbiAgICAgICAgdGV4dC1hbGlnbjogY2VudGVyO1xuICAgIH1cblxuICAgICZfX2dyaWQge1xuICAgICAgICBkaXNwbGF5OiBncmlkO1xuICAgICAgICBncmlkLXRlbXBsYXRlLWNvbHVtbnM6IHJlcGVhdCgxMCwgNTBweCk7XG4gICAgICAgIGdyaWQtdGVtcGxhdGUtcm93czogcmVwZWF0KDEwLCA1MHB4KTtcbiAgICAgICAgY3Vyc29yOiBwb2ludGVyO1xuXG4gICAgICAgICYtLWRpc2FibGVkIHtcbiAgICAgICAgICAgIGN1cnNvcjogbm90LWFsbG93ZWQ7XG4gICAgICAgICAgICBwb2ludGVyLWV2ZW50czogbm9uZTtcbiAgICAgICAgfVxuICAgIH1cblxuICAgICZfX3N0YXR1c1dyYXBwZXIge1xuICAgICAgICBkaXNwbGF5OiBmbGV4O1xuICAgICAgICBmbGV4LWRpcmVjdGlvbjogY29sdW1uO1xuICAgICAgICBqdXN0aWZ5LWNvbnRlbnQ6IGNlbnRlcjtcbiAgICAgICAgaGVpZ2h0OiA1MDBweDtcbiAgICB9XG5cbiAgICAmX19wbGF5QWdhaW4sICZfX3N0YXR1cyB7XG4gICAgICAgIGRpc3BsYXk6IGZsZXg7XG4gICAgICAgIGp1c3RpZnktY29udGVudDogY2VudGVyO1xuICAgICAgICBhbGlnbi1pdGVtczogY2VudGVyO1xuICAgIH1cblxuICAgICZfX3N0YXR1c0hlYWRlciB7XG4gICAgICAgIGZvbnQtc2l6ZTogM3JlbTtcbiAgICAgICAgbWFyZ2luLXJpZ2h0OiAxcmVtO1xuICAgIH1cblxuICAgICZfX3BsYXlBZ2FpbiB7XG4gICAgICAgIGN1cnNvcjogcG9pbnRlcjtcbiAgICAgICAgbWFyZ2luLXRvcDogMnJlbTtcbiAgICAgICAgcGFkZGluZzogLjVyZW0gMDtcbiAgICAgICAgdHJhbnNpdGlvbjogYm9yZGVyIC4zcyBlYXNlLWluO1xuICAgICAgICBib3JkZXI6IDFweCBzb2xpZCB0cmFuc3BhcmVudDsgXG5cbiAgICAgICAgJjpob3ZlciB7XG4gICAgICAgICAgICBib3JkZXI6IDFweCBzb2xpZDsgICAgXG4gICAgICAgIH1cbiAgICB9XG5cbiAgICAmX19wbGF5QWdhaW5UZXh0IHtcbiAgICAgICAgbWFyZ2luLXJpZ2h0OiAxcmVtO1xuICAgIH1cblxuICAgICZfX2NlbGwge1xuICAgICAgICBkaXNwbGF5OiBmbGV4O1xuICAgICAgICBqdXN0aWZ5LWNvbnRlbnQ6IGNlbnRlcjtcbiAgICAgICAgYWxpZ24taXRlbXM6IGNlbnRlcjtcbiAgICAgICAgdHJhbnNpdGlvbjogYWxsIC4zcyBlYXNlLWluO1xuICAgICAgICBib3JkZXI6IDFweCBzb2xpZCBAd2hpdGU7XG4gICAgfVxufVxuXG4ubnVtYmVyIHtcbiAgICBmb250LXNpemU6IDJyZW07XG59XG5cbi5yZXZlYWxlZCB7XG4gICAgYmFja2dyb3VuZDogd2hpdGU7XG4gICAgd2lkdGg6IDEwMCU7XG4gICAgaGVpZ2h0OiAxMDAlO1xufVxuXG4udW5yZXZlYWxlZCB7XG4gICAgXG59XG5cbi5hY3Rpb25zIHtcbiAgICBkaXNwbGF5OiBmbGV4O1xuICAgIGFsaWduLWl0ZW1zOiBjZW50ZXI7XG4gICAganVzdGlmeS1jb250ZW50OiBzcGFjZS1hcm91bmQ7XG4gICAgcGFkZGluZzogMXJlbSAwO1xuICAgIHdpZHRoOiA1MDBweDtcbiAgICBtYXJnaW46IGF1dG87XG5cbiAgICAmX19yZXNldCwgJl9fZ2l2ZS11cCB7XG4gICAgICAgIGN1cnNvcjogcG9pbnRlcjtcbiAgICB9XG5cbiAgICAmX19naXZlLXVwIHtcbiAgICAgICAgZm9udC1zaXplOiAycmVtO1xuICAgICAgICBwYWRkaW5nOiAuNXJlbTtcbiAgICAgICAgYm9yZGVyOiAxcHggc29saWQgQHdoaXRlO1xuXG4gICAgICAgICYtLWRpc2FibGVkIHtcbiAgICAgICAgICAgIGNvbG9yOiBncmF5O1xuICAgICAgICAgICAgYm9yZGVyLWNvbG9yOiBncmF5O1xuICAgICAgICAgICAgcG9pbnRlci1ldmVudHM6IG5vbmU7XG4gICAgICAgICAgICBjdXJzb3I6IG5vbmU7XG4gICAgICAgIH1cbiAgICB9XG59IiwiOmhvc3Qge1xuICBkaXNwbGF5OiBibG9jaztcbiAgaGVpZ2h0OiAxMDAlO1xufVxuLmdhbWUge1xuICBkaXNwbGF5OiBmbGV4O1xuICBqdXN0aWZ5LWNvbnRlbnQ6IGNlbnRlcjtcbn1cbi5nYW1lX19oZWFkZXIge1xuICBmb250LXNpemU6IDRyZW07XG4gIHBhZGRpbmc6IDFyZW0gMDtcbiAgdGV4dC1hbGlnbjogY2VudGVyO1xufVxuLmdhbWVfX2dyaWQge1xuICBkaXNwbGF5OiBncmlkO1xuICBncmlkLXRlbXBsYXRlLWNvbHVtbnM6IHJlcGVhdCgxMCwgNTBweCk7XG4gIGdyaWQtdGVtcGxhdGUtcm93czogcmVwZWF0KDEwLCA1MHB4KTtcbiAgY3Vyc29yOiBwb2ludGVyO1xufVxuLmdhbWVfX2dyaWQtLWRpc2FibGVkIHtcbiAgY3Vyc29yOiBub3QtYWxsb3dlZDtcbiAgcG9pbnRlci1ldmVudHM6IG5vbmU7XG59XG4uZ2FtZV9fc3RhdHVzV3JhcHBlciB7XG4gIGRpc3BsYXk6IGZsZXg7XG4gIGZsZXgtZGlyZWN0aW9uOiBjb2x1bW47XG4gIGp1c3RpZnktY29udGVudDogY2VudGVyO1xuICBoZWlnaHQ6IDUwMHB4O1xufVxuLmdhbWVfX3BsYXlBZ2Fpbixcbi5nYW1lX19zdGF0dXMge1xuICBkaXNwbGF5OiBmbGV4O1xuICBqdXN0aWZ5LWNvbnRlbnQ6IGNlbnRlcjtcbiAgYWxpZ24taXRlbXM6IGNlbnRlcjtcbn1cbi5nYW1lX19zdGF0dXNIZWFkZXIge1xuICBmb250LXNpemU6IDNyZW07XG4gIG1hcmdpbi1yaWdodDogMXJlbTtcbn1cbi5nYW1lX19wbGF5QWdhaW4ge1xuICBjdXJzb3I6IHBvaW50ZXI7XG4gIG1hcmdpbi10b3A6IDJyZW07XG4gIHBhZGRpbmc6IDAuNXJlbSAwO1xuICB0cmFuc2l0aW9uOiBib3JkZXIgMC4zcyBlYXNlLWluO1xuICBib3JkZXI6IDFweCBzb2xpZCB0cmFuc3BhcmVudDtcbn1cbi5nYW1lX19wbGF5QWdhaW46aG92ZXIge1xuICBib3JkZXI6IDFweCBzb2xpZDtcbn1cbi5nYW1lX19wbGF5QWdhaW5UZXh0IHtcbiAgbWFyZ2luLXJpZ2h0OiAxcmVtO1xufVxuLmdhbWVfX2NlbGwge1xuICBkaXNwbGF5OiBmbGV4O1xuICBqdXN0aWZ5LWNvbnRlbnQ6IGNlbnRlcjtcbiAgYWxpZ24taXRlbXM6IGNlbnRlcjtcbiAgdHJhbnNpdGlvbjogYWxsIDAuM3MgZWFzZS1pbjtcbiAgYm9yZGVyOiAxcHggc29saWQgI0ZGRkZGRjk5O1xufVxuLm51bWJlciB7XG4gIGZvbnQtc2l6ZTogMnJlbTtcbn1cbi5yZXZlYWxlZCB7XG4gIGJhY2tncm91bmQ6IHdoaXRlO1xuICB3aWR0aDogMTAwJTtcbiAgaGVpZ2h0OiAxMDAlO1xufVxuLmFjdGlvbnMge1xuICBkaXNwbGF5OiBmbGV4O1xuICBhbGlnbi1pdGVtczogY2VudGVyO1xuICBqdXN0aWZ5LWNvbnRlbnQ6IHNwYWNlLWFyb3VuZDtcbiAgcGFkZGluZzogMXJlbSAwO1xuICB3aWR0aDogNTAwcHg7XG4gIG1hcmdpbjogYXV0bztcbn1cbi5hY3Rpb25zX19yZXNldCxcbi5hY3Rpb25zX19naXZlLXVwIHtcbiAgY3Vyc29yOiBwb2ludGVyO1xufVxuLmFjdGlvbnNfX2dpdmUtdXAge1xuICBmb250LXNpemU6IDJyZW07XG4gIHBhZGRpbmc6IDAuNXJlbTtcbiAgYm9yZGVyOiAxcHggc29saWQgI0ZGRkZGRjk5O1xufVxuLmFjdGlvbnNfX2dpdmUtdXAtLWRpc2FibGVkIHtcbiAgY29sb3I6IGdyYXk7XG4gIGJvcmRlci1jb2xvcjogZ3JheTtcbiAgcG9pbnRlci1ldmVudHM6IG5vbmU7XG4gIGN1cnNvcjogbm9uZTtcbn1cbiJdfQ== */"], data: { animation: [
            Object(_angular_animations__WEBPACK_IMPORTED_MODULE_2__["trigger"])('enterTrigger', [
                //transition(':leave', [style({opacity: '0'}), animate('1500ms')]),
                Object(_angular_animations__WEBPACK_IMPORTED_MODULE_2__["transition"])('void => *', [Object(_angular_animations__WEBPACK_IMPORTED_MODULE_2__["style"])({ opacity: '0' }), Object(_angular_animations__WEBPACK_IMPORTED_MODULE_2__["animate"])('1500ms')])
            ])
        ] } });
/*@__PURE__*/ (function () { _angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵsetClassMetadata"](GameComponent, [{
        type: _angular_core__WEBPACK_IMPORTED_MODULE_0__["Component"],
        args: [{
                selector: 'game',
                templateUrl: './game.component.html',
                styleUrls: ['./game.component.less'],
                animations: [
                    Object(_angular_animations__WEBPACK_IMPORTED_MODULE_2__["trigger"])('enterTrigger', [
                        //transition(':leave', [style({opacity: '0'}), animate('1500ms')]),
                        Object(_angular_animations__WEBPACK_IMPORTED_MODULE_2__["transition"])('void => *', [Object(_angular_animations__WEBPACK_IMPORTED_MODULE_2__["style"])({ opacity: '0' }), Object(_angular_animations__WEBPACK_IMPORTED_MODULE_2__["animate"])('1500ms')])
                    ])
                ]
            }]
    }], function () { return [{ type: src_minesweeper_service_minesweeper_service__WEBPACK_IMPORTED_MODULE_3__["MinesweeperService"] }]; }, null); })();


/***/ }),

/***/ "./src/environments/environment.ts":
/*!*****************************************!*\
  !*** ./src/environments/environment.ts ***!
  \*****************************************/
/*! exports provided: environment */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "environment", function() { return environment; });
const environment = {
    production: false
};


/***/ }),

/***/ "./src/main.ts":
/*!*********************!*\
  !*** ./src/main.ts ***!
  \*********************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! @angular/core */ "./node_modules/@angular/core/__ivy_ngcc__/fesm2015/core.js");
/* harmony import */ var _environments_environment__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./environments/environment */ "./src/environments/environment.ts");
/* harmony import */ var _app_app_module__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./app/app.module */ "./src/app/app.module.ts");
/* harmony import */ var _angular_platform_browser__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! @angular/platform-browser */ "./node_modules/@angular/platform-browser/__ivy_ngcc__/fesm2015/platform-browser.js");




if (_environments_environment__WEBPACK_IMPORTED_MODULE_1__["environment"].production) {
    Object(_angular_core__WEBPACK_IMPORTED_MODULE_0__["enableProdMode"])();
}
_angular_platform_browser__WEBPACK_IMPORTED_MODULE_3__["platformBrowser"]().bootstrapModule(_app_app_module__WEBPACK_IMPORTED_MODULE_2__["AppModule"])
    .catch(err => console.error(err));


/***/ }),

/***/ "./src/minesweeper-service/minesweeper.service.ts":
/*!********************************************************!*\
  !*** ./src/minesweeper-service/minesweeper.service.ts ***!
  \********************************************************/
/*! exports provided: MinesweeperService */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "MinesweeperService", function() { return MinesweeperService; });
/* harmony import */ var tslib__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! tslib */ "./node_modules/tslib/tslib.es6.js");
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/core */ "./node_modules/@angular/core/__ivy_ngcc__/fesm2015/core.js");


let MinesweeperService = class MinesweeperService {
    constructor() {
        this.getNeighbors = (row, column) => {
            return [
                {
                    row: row - 1,
                    column: column
                },
                {
                    row: row - 1,
                    column: column - 1
                },
                {
                    row: row - 1,
                    column: column + 1
                },
                {
                    row: row + 1,
                    column
                },
                {
                    row: row + 1,
                    column: column - 1
                },
                {
                    row: row + 1,
                    column: column + 1
                },
                {
                    row,
                    column: column - 1
                },
                {
                    row,
                    column: column + 1
                }
            ];
        };
        this.computeNeighbors = (board, cell) => {
            const relativeNeighbors = [{
                    row: cell.row,
                    column: cell.column + 1
                },
                {
                    row: cell.row,
                    column: cell.column - 1
                },
                {
                    row: cell.row + 1,
                    column: cell.column
                },
                {
                    row: cell.row - 1,
                    column: cell.column
                }];
            const neighbors = board.filter(cell => relativeNeighbors.some(pr => pr.column === cell.column && pr.row === cell.row));
            return neighbors.map(pr => board
                .find(cell => cell.row === pr.row && cell.column === pr.column));
        };
        this.uncoverNeighbors = (board, cell) => {
            let neighbors = this.computeNeighbors(board, cell);
            let checkedNeighbors = [];
            let result = [];
            while (neighbors.length) {
                const firstCell = neighbors.pop();
                const isChecked = checkedNeighbors.find(cell => cell.id === firstCell.id);
                if (isChecked || firstCell.hasBomb) {
                    continue;
                }
                if (!firstCell.bombCount) {
                    neighbors = [...neighbors, ...this.computeNeighbors(board, firstCell)];
                }
                checkedNeighbors = [...checkedNeighbors, firstCell];
                result = [...result, firstCell];
            }
            return result;
        };
    }
    initializeBoard() {
        return new Promise((resolve, reject) => {
            const rows = 10;
            const columns = 10;
            let board = Array(rows * columns).fill(0).map((pr, index) => {
                return {
                    id: index,
                    row: Math.floor(index / rows),
                    column: index % columns,
                    hasBomb: Boolean(Math.floor(Math.random() * 1.04)),
                    isFlagged: false,
                    isRevealed: false,
                    bombCount: 0
                };
            });
            board = board.map(pr => {
                const computedNeighbors = this.getNeighbors(pr.row, pr.column);
                let bombCount = null;
                if (!pr.hasBomb) {
                    bombCount = board.filter(cell => computedNeighbors
                        .some(pr => pr.row === cell.row && pr.column === cell.column))
                        .reduce((acc, cell) => acc + Number(cell.hasBomb), 0);
                }
                return Object.assign(Object.assign({}, pr), { bombCount });
            });
            resolve(board);
        });
    }
};
MinesweeperService = Object(tslib__WEBPACK_IMPORTED_MODULE_0__["__decorate"])([
    Object(_angular_core__WEBPACK_IMPORTED_MODULE_1__["Inject"])({
        providedIn: 'root'
    })
], MinesweeperService);



/***/ }),

/***/ 0:
/*!***************************!*\
  !*** multi ./src/main.ts ***!
  \***************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

module.exports = __webpack_require__(/*! C:\Users\Józef Podlecki\Documents\repos\Minesweeper\src\main.ts */"./src/main.ts");


/***/ })

},[[0,"runtime","vendor"]]]);
//# sourceMappingURL=main-es2015.js.map
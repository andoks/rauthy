import{S as Oe,i as Te,s as Fe,a as A,y as M,W as Ve,h as g,c as W,z as B,b as N,A as D,g as b,d as q,B as O,o as je,aa as Ge,w as E,N as k,k as ue,q as T,l as pe,m as de,r as F,n as be,G as I,v as Ee,f as ke,Q as P,u as Ye,H as He}from"../chunks/index.fdf84907.js";import{c as ze,a as he,I as Qe}from"../chunks/Input.78a87232.js";import{a as Je,e as Ke,h as Ue,Y as Xe}from"../chunks/dataFetching.f2a9713f.js";import{L as Ze}from"../chunks/Loading.a4841b1e.js";import{B as Pe}from"../chunks/Button.80ee52cd.js";import{P as xe}from"../chunks/PasswordPolicy.6f8aa5cb.js";import{P as qe}from"../chunks/PasswordInput.969a95d0.js";import{W as es}from"../chunks/WebauthnRequest.4fa59eab.js";import{B as ss}from"../chunks/BrowserCheck.ea95e5bb.js";const{document:Ie}=Ge;function Re(o){let n,e;return n=new Ze({}),{c(){M(n.$$.fragment)},l(r){B(n.$$.fragment,r)},m(r,i){D(n,r,i),e=!0},i(r){e||(b(n.$$.fragment,r),e=!0)},o(r){q(n.$$.fragment,r),e=!1},d(r){O(n,r)}}}function Se(o){let n,e,r;function i(p){o[15](p)}let l={purpose:"PasswordReset",onSuccess:o[14],onError:o[13]};return o[9]!==void 0&&(l.data=o[9]),n=new es({props:l}),E.push(()=>k(n,"data",i)),{c(){M(n.$$.fragment)},l(p){B(n.$$.fragment,p)},m(p,f){D(n,p,f),r=!0},p(p,f){const $={};!e&&f[0]&512&&(e=!0,$.data=p[9],P(()=>e=!1)),n.$set($)},i(p){r||(b(n.$$.fragment,p),r=!0)},o(p){q(n.$$.fragment,p),r=!1},d(p){O(n,p)}}}function os(o){let n;return{c(){n=T("E-MAIL")},l(e){n=F(e,"E-MAIL")},m(e,r){N(e,n,r)},d(e){e&&g(n)}}}function ns(o){let n;return{c(){n=T("PASSWORD")},l(e){n=F(e,"PASSWORD")},m(e,r){N(e,n,r)},d(e){e&&g(n)}}}function rs(o){let n;return{c(){n=T("PASSWORD CONFIRM")},l(e){n=F(e,"PASSWORD CONFIRM")},m(e,r){N(e,n,r)},d(e){e&&g(n)}}}function ts(o){let n;return{c(){n=T("GENERATE")},l(e){n=F(e,"GENERATE")},m(e,r){N(e,n,r)},d(e){e&&g(n)}}}function as(o){let n;return{c(){n=T("RESET")},l(e){n=F(e,"RESET")},m(e,r){N(e,n,r)},d(e){e&&g(n)}}}function is(o){let n,e;return{c(){n=ue("div"),e=T(o[5]),this.h()},l(r){n=pe(r,"DIV",{class:!0});var i=de(n);e=F(i,o[5]),i.forEach(g),this.h()},h(){be(n,"class","err svelte-1n46pmb")},m(r,i){N(r,n,i),I(n,e)},p(r,i){i[0]&32&&Ye(e,r[5])},d(r){r&&g(n)}}}function us(o){let n,e,r,i;return{c(){n=ue("div"),e=T("The password has been updated successfully."),r=ue("br"),i=T(`
                You can close this window now.`),this.h()},l(l){n=pe(l,"DIV",{class:!0});var p=de(n);e=F(p,"The password has been updated successfully."),r=pe(p,"BR",{}),i=F(p,`
                You can close this window now.`),p.forEach(g),this.h()},h(){be(n,"class","success svelte-1n46pmb")},m(l,p){N(l,n,p),I(n,e),I(n,r),I(n,i)},p:He,d(l){l&&g(n)}}}function ps(o){let n,e,r,i,l,p,f,$,V,Y,j,m,v,u,w,C,K,U,H,X,y,Z,x,ee,se,S,oe,R,ne,re,z,_=!o[2]&&Re(),d=o[9]&&Se(o);function _e(s){o[16](s)}function t(s){o[17](s)}function c(s){o[18](s)}let L={};o[7]!==void 0&&(L.accepted=o[7]),o[1]!==void 0&&(L.policy=o[1]),o[0].password!==void 0&&(L.password=o[0].password),f=new xe({props:L}),E.push(()=>k(f,"accepted",_e)),E.push(()=>k(f,"policy",t)),E.push(()=>k(f,"password",c));function Q(s){o[19](s)}function ce(s){o[20](s)}let me={type:"email",autocomplete:"email",disabled:o[3],placeholder:"E-Mail",width:ge,$$slots:{default:[os]},$$scope:{ctx:o}};o[0].email!==void 0&&(me.value=o[0].email),o[10].email!==void 0&&(me.error=o[10].email),m=new Qe({props:me}),E.push(()=>k(m,"value",Q)),E.push(()=>k(m,"error",ce));function ve(s){o[21](s)}function Le(s){o[22](s)}function Ae(s){o[23](s)}let le={autocomplete:"new-password",placeholder:"Password",width:ge,$$slots:{default:[ns]},$$scope:{ctx:o}};o[0].password!==void 0&&(le.value=o[0].password),o[10].password!==void 0&&(le.error=o[10].password),o[8]!==void 0&&(le.showCopy=o[8]),C=new qe({props:le}),E.push(()=>k(C,"value",ve)),E.push(()=>k(C,"error",Le)),E.push(()=>k(C,"showCopy",Ae));function We(s){o[24](s)}function Me(s){o[25](s)}function Be(s){o[26](s)}let fe={autocomplete:"new-password",placeholder:"Confirm Password",width:ge,$$slots:{default:[rs]},$$scope:{ctx:o}};o[0].passwordConfirm!==void 0&&(fe.value=o[0].passwordConfirm),o[10].passwordConfirm!==void 0&&(fe.error=o[10].passwordConfirm),o[8]!==void 0&&(fe.showCopy=o[8]),y=new qe({props:fe}),E.push(()=>k(y,"value",We)),E.push(()=>k(y,"error",Me)),E.push(()=>k(y,"showCopy",Be)),S=new Pe({props:{width:Ne,level:3,$$slots:{default:[ts]},$$scope:{ctx:o}}}),S.$on("click",o[11]);function De(s){o[27](s)}let $e={width:Ne,level:2,$$slots:{default:[as]},$$scope:{ctx:o}};o[4]!==void 0&&($e.isLoading=o[4]),R=new Pe({props:$e}),E.push(()=>k(R,"isLoading",De)),R.$on("click",o[12]);function Ce(s,a){if(s[6])return us;if(s[5])return is}let J=Ce(o),h=J&&J(o);return{c(){_&&_.c(),n=A(),d&&d.c(),e=A(),r=ue("div"),i=ue("h1"),l=T("Password Reset"),p=A(),M(f.$$.fragment),j=A(),M(m.$$.fragment),w=A(),M(C.$$.fragment),X=A(),M(y.$$.fragment),se=A(),M(S.$$.fragment),oe=A(),M(R.$$.fragment),re=A(),h&&h.c(),this.h()},l(s){_&&_.l(s),n=W(s),d&&d.l(s),e=W(s),r=pe(s,"DIV",{class:!0});var a=de(r);i=pe(a,"H1",{});var G=de(i);l=F(G,"Password Reset"),G.forEach(g),p=W(a),B(f.$$.fragment,a),j=W(a),B(m.$$.fragment,a),w=W(a),B(C.$$.fragment,a),X=W(a),B(y.$$.fragment,a),se=W(a),B(S.$$.fragment,a),oe=W(a),B(R.$$.fragment,a),re=W(a),h&&h.l(a),a.forEach(g),this.h()},h(){be(r,"class","container svelte-1n46pmb")},m(s,a){_&&_.m(s,a),N(s,n,a),d&&d.m(s,a),N(s,e,a),N(s,r,a),I(r,i),I(i,l),I(r,p),D(f,r,null),I(r,j),D(m,r,null),I(r,w),D(C,r,null),I(r,X),D(y,r,null),I(r,se),D(S,r,null),I(r,oe),D(R,r,null),I(r,re),h&&h.m(r,null),z=!0},p(s,a){s[2]?_&&(Ee(),q(_,1,1,()=>{_=null}),ke()):_?a[0]&4&&b(_,1):(_=Re(),_.c(),b(_,1),_.m(n.parentNode,n)),s[9]?d?(d.p(s,a),a[0]&512&&b(d,1)):(d=Se(s),d.c(),b(d,1),d.m(e.parentNode,e)):d&&(Ee(),q(d,1,1,()=>{d=null}),ke());const G={};!$&&a[0]&128&&($=!0,G.accepted=s[7],P(()=>$=!1)),!V&&a[0]&2&&(V=!0,G.policy=s[1],P(()=>V=!1)),!Y&&a[0]&1&&(Y=!0,G.password=s[0].password,P(()=>Y=!1)),f.$set(G);const te={};a[0]&8&&(te.disabled=s[3]),a[1]&2&&(te.$$scope={dirty:a,ctx:s}),!v&&a[0]&1&&(v=!0,te.value=s[0].email,P(()=>v=!1)),!u&&a[0]&1024&&(u=!0,te.error=s[10].email,P(()=>u=!1)),m.$set(te);const ae={};a[1]&2&&(ae.$$scope={dirty:a,ctx:s}),!K&&a[0]&1&&(K=!0,ae.value=s[0].password,P(()=>K=!1)),!U&&a[0]&1024&&(U=!0,ae.error=s[10].password,P(()=>U=!1)),!H&&a[0]&256&&(H=!0,ae.showCopy=s[8],P(()=>H=!1)),C.$set(ae);const ie={};a[1]&2&&(ie.$$scope={dirty:a,ctx:s}),!Z&&a[0]&1&&(Z=!0,ie.value=s[0].passwordConfirm,P(()=>Z=!1)),!x&&a[0]&1024&&(x=!0,ie.error=s[10].passwordConfirm,P(()=>x=!1)),!ee&&a[0]&256&&(ee=!0,ie.showCopy=s[8],P(()=>ee=!1)),y.$set(ie);const ye={};a[1]&2&&(ye.$$scope={dirty:a,ctx:s}),S.$set(ye);const we={};a[1]&2&&(we.$$scope={dirty:a,ctx:s}),!ne&&a[0]&16&&(ne=!0,we.isLoading=s[4],P(()=>ne=!1)),R.$set(we),J===(J=Ce(s))&&h?h.p(s,a):(h&&h.d(1),h=J&&J(s),h&&(h.c(),h.m(r,null)))},i(s){z||(b(_),b(d),b(f.$$.fragment,s),b(m.$$.fragment,s),b(C.$$.fragment,s),b(y.$$.fragment,s),b(S.$$.fragment,s),b(R.$$.fragment,s),z=!0)},o(s){q(_),q(d),q(f.$$.fragment,s),q(m.$$.fragment,s),q(C.$$.fragment,s),q(y.$$.fragment,s),q(S.$$.fragment,s),q(R.$$.fragment,s),z=!1},d(s){_&&_.d(s),s&&g(n),d&&d.d(s),s&&g(e),s&&g(r),O(f),O(m),O(C),O(y),O(S),O(R),h&&h.d()}}}function ls(o){let n,e,r;return e=new ss({props:{$$slots:{default:[ps]},$$scope:{ctx:o}}}),{c(){n=A(),M(e.$$.fragment),this.h()},l(i){Ve("svelte-15vejor",Ie.head).forEach(g),n=W(i),B(e.$$.fragment,i),this.h()},h(){Ie.title="Password Reset"},m(i,l){N(i,n,l),D(e,i,l),r=!0},p(i,l){const p={};l[0]&2047|l[1]&2&&(p.$$scope={dirty:l,ctx:i}),e.$set(p)},i(i){r||(b(e.$$.fragment,i),r=!0)},o(i){q(e.$$.fragment,i),r=!1},d(i){i&&g(n),O(e,i)}}}const Ne=150,ge="320px";function fs(o,n,e){let r="",i,l=!1,p=!1,f=!1,$="",V="",Y=!1,j=!1,m=!1,v,u={email:"",password:"",passwordConfirm:""},w={};const C=ze().shape({email:he().required("E-Mail is required").email("Bad E-Mail format"),password:he().required("Password is required"),passwordConfirm:he().required("Confirm Password is required")});je(async()=>{const t=document.getElementsByName("rauthy-data")[0].id,c=[];t.split(",").forEach(Q=>c.push(Q)),e(1,i={length_min:Number.parseInt(c[0]),length_max:Number.parseInt(c[1]),include_lower_case:Number.parseInt(c[2]),include_upper_case:Number.parseInt(c[3]),include_digits:Number.parseInt(c[4]),include_special:Number.parseInt(c[5]),not_recently_used:Number.parseInt(c[6])});let L=c[7];L&&L!=="undefined"&&(e(0,u.email=L,u),e(3,p=!0)),r=window.document.getElementsByName("rauthy-csrf-token")[0].id,V=window.location.href.split("/users/")[1].split("/")[0],e(2,l=!0)});function K(){const t=i.length_min>24?i.length_min:24;let c=Je(t,i.include_lower_case,i.include_upper_case,i.include_digits,i.include_special);e(0,u.password=c,u),e(0,u.passwordConfirm=c,u)}async function U(){try{await C.validate(u,{abortEarly:!1}),e(10,w={})}catch(t){e(10,w=Ke(t));return}if(j){if(u.password!==u.passwordConfirm){e(5,$="Passwords do not match");return}else e(5,$="");if(p){let t=await Ue(V,{purpose:"PasswordReset"}),c=await t.json();if(!t.ok){e(5,$=c.message),e(4,f=!1);return}if(c.user_id!==V){e(5,$="MFA user ID does not match - this should never happen"),e(4,f=!1);return}e(9,v=c)}else await H()}}async function H(t){e(4,f=!0);const c=window.location.href.split("/reset/")[1],L={email:u.email,password:u.password,magic_link_id:c,mfa_code:t},Q=await Xe(V,L,r);if(Q.ok)e(5,$=""),e(6,Y=!0);else{const ce=await Q.json();e(5,$=ce.message)}e(4,f=!1)}function X(){e(9,v=void 0)}function y(t){t&&(e(9,v=void 0),H(t.code))}function Z(t){v=t,e(9,v)}function x(t){j=t,e(7,j)}function ee(t){i=t,e(1,i)}function se(t){o.$$.not_equal(u.password,t)&&(u.password=t,e(0,u))}function S(t){o.$$.not_equal(u.email,t)&&(u.email=t,e(0,u))}function oe(t){o.$$.not_equal(w.email,t)&&(w.email=t,e(10,w))}function R(t){o.$$.not_equal(u.password,t)&&(u.password=t,e(0,u))}function ne(t){o.$$.not_equal(w.password,t)&&(w.password=t,e(10,w))}function re(t){m=t,e(8,m),e(0,u)}function z(t){o.$$.not_equal(u.passwordConfirm,t)&&(u.passwordConfirm=t,e(0,u))}function _(t){o.$$.not_equal(w.passwordConfirm,t)&&(w.passwordConfirm=t,e(10,w))}function d(t){m=t,e(8,m),e(0,u)}function _e(t){f=t,e(4,f)}return o.$$.update=()=>{var t;o.$$.dirty[0]&1&&((t=u.password)==null?void 0:t.length)>0&&u.password===u.passwordConfirm&&e(8,m=!0)},[u,i,l,p,f,$,Y,j,m,v,w,K,U,X,y,Z,x,ee,se,S,oe,R,ne,re,z,_,d,_e]}class Cs extends Oe{constructor(n){super(),Te(this,n,fs,ls,Fe,{},null,[-1,-1])}}export{Cs as component};
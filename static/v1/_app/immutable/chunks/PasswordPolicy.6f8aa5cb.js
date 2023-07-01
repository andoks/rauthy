import{S as ae,i as ce,s as ne,e as Z,b as D,H as $,h as p,k as L,q as w,a as P,l as M,m as C,r as b,c as q,p as re,n as V,V as I,G as r,u as B}from"./index.fdf84907.js";function x(n){var X;let e,a,t,f,c,s,_,i="Min length: ",l,u,d=n[0].length_min+"",A,N,S,W="Max length: ",Q,R,G=((X=n[0])==null?void 0:X.length_max)+"",U,T,j,z,F,J,h=n[0].include_lower_case!==-1&&ee(n),m=n[0].include_upper_case!==-1&&le(n),v=n[0].include_digits!==-1&&ie(n),y=n[0].include_special!==-1&&te(n),g=n[0].not_recently_used!==-1&&se(n);return{c(){e=L("div"),a=L("span"),t=L("b"),f=w("Password Policy:"),c=P(),s=L("ul"),_=L("li"),l=w(i),u=P(),A=w(d),N=P(),S=L("li"),Q=w(W),R=P(),U=w(G),T=P(),h&&h.c(),j=P(),m&&m.c(),z=P(),v&&v.c(),F=P(),y&&y.c(),J=P(),g&&g.c(),this.h()},l(o){e=M(o,"DIV",{class:!0});var E=C(e);a=M(E,"SPAN",{style:!0});var H=C(a);t=M(H,"B",{});var Y=C(t);f=b(Y,"Password Policy:"),Y.forEach(p),H.forEach(p),c=q(E),s=M(E,"UL",{});var k=C(s);_=M(k,"LI",{class:!0});var K=C(_);l=b(K,i),u=q(K),A=b(K,d),K.forEach(p),N=q(k),S=M(k,"LI",{class:!0});var O=C(S);Q=b(O,W),R=q(O),U=b(O,G),O.forEach(p),T=q(k),h&&h.l(k),j=q(k),m&&m.l(k),z=q(k),v&&v.l(k),F=q(k),y&&y.l(k),J=q(k),g&&g.l(k),k.forEach(p),E.forEach(p),this.h()},h(){re(a,"margin-left","20px"),V(_,"class","li svelte-qyge6w"),I(_,"policyErr",!!n[1][0]),V(S,"class","li svelte-qyge6w"),I(S,"policyErr",!!n[1][1]),V(e,"class","policyContainer svelte-qyge6w")},m(o,E){D(o,e,E),r(e,a),r(a,t),r(t,f),r(e,c),r(e,s),r(s,_),r(_,l),r(_,u),r(_,A),r(s,N),r(s,S),r(S,Q),r(S,R),r(S,U),r(s,T),h&&h.m(s,null),r(s,j),m&&m.m(s,null),r(s,z),v&&v.m(s,null),r(s,F),y&&y.m(s,null),r(s,J),g&&g.m(s,null)},p(o,E){var H;E&1&&d!==(d=o[0].length_min+"")&&B(A,d),E&2&&I(_,"policyErr",!!o[1][0]),E&1&&G!==(G=((H=o[0])==null?void 0:H.length_max)+"")&&B(U,G),E&2&&I(S,"policyErr",!!o[1][1]),o[0].include_lower_case!==-1?h?h.p(o,E):(h=ee(o),h.c(),h.m(s,j)):h&&(h.d(1),h=null),o[0].include_upper_case!==-1?m?m.p(o,E):(m=le(o),m.c(),m.m(s,z)):m&&(m.d(1),m=null),o[0].include_digits!==-1?v?v.p(o,E):(v=ie(o),v.c(),v.m(s,F)):v&&(v.d(1),v=null),o[0].include_special!==-1?y?y.p(o,E):(y=te(o),y.c(),y.m(s,J)):y&&(y.d(1),y=null),o[0].not_recently_used!==-1?g?g.p(o,E):(g=se(o),g.c(),g.m(s,null)):g&&(g.d(1),g=null)},d(o){o&&p(e),h&&h.d(),m&&m.d(),v&&v.d(),y&&y.d(),g&&g.d()}}}function ee(n){var _;let e,a="Min lowercase letters: ",t,f,c=((_=n[0])==null?void 0:_.include_lower_case)+"",s;return{c(){e=L("li"),t=w(a),f=P(),s=w(c),this.h()},l(i){e=M(i,"LI",{class:!0});var l=C(e);t=b(l,a),f=q(l),s=b(l,c),l.forEach(p),this.h()},h(){V(e,"class","li svelte-qyge6w"),I(e,"policyErr",!!n[1][2])},m(i,l){D(i,e,l),r(e,t),r(e,f),r(e,s)},p(i,l){var u;l&1&&c!==(c=((u=i[0])==null?void 0:u.include_lower_case)+"")&&B(s,c),l&2&&I(e,"policyErr",!!i[1][2])},d(i){i&&p(e)}}}function le(n){var _;let e,a="Min uppercase letters: ",t,f,c=((_=n[0])==null?void 0:_.include_upper_case)+"",s;return{c(){e=L("li"),t=w(a),f=P(),s=w(c),this.h()},l(i){e=M(i,"LI",{class:!0});var l=C(e);t=b(l,a),f=q(l),s=b(l,c),l.forEach(p),this.h()},h(){V(e,"class","li svelte-qyge6w"),I(e,"policyErr",!!n[1][3])},m(i,l){D(i,e,l),r(e,t),r(e,f),r(e,s)},p(i,l){var u;l&1&&c!==(c=((u=i[0])==null?void 0:u.include_upper_case)+"")&&B(s,c),l&2&&I(e,"policyErr",!!i[1][3])},d(i){i&&p(e)}}}function ie(n){var _;let e,a="Min digits: ",t,f,c=((_=n[0])==null?void 0:_.include_digits)+"",s;return{c(){e=L("li"),t=w(a),f=P(),s=w(c),this.h()},l(i){e=M(i,"LI",{class:!0});var l=C(e);t=b(l,a),f=q(l),s=b(l,c),l.forEach(p),this.h()},h(){V(e,"class","li svelte-qyge6w"),I(e,"policyErr",!!n[1][4])},m(i,l){D(i,e,l),r(e,t),r(e,f),r(e,s)},p(i,l){var u;l&1&&c!==(c=((u=i[0])==null?void 0:u.include_digits)+"")&&B(s,c),l&2&&I(e,"policyErr",!!i[1][4])},d(i){i&&p(e)}}}function te(n){var _;let e,a="Min special characters: ",t,f,c=((_=n[0])==null?void 0:_.include_special)+"",s;return{c(){e=L("li"),t=w(a),f=P(),s=w(c),this.h()},l(i){e=M(i,"LI",{class:!0});var l=C(e);t=b(l,a),f=q(l),s=b(l,c),l.forEach(p),this.h()},h(){V(e,"class","li svelte-qyge6w"),I(e,"policyErr",!!n[1][5])},m(i,l){D(i,e,l),r(e,t),r(e,f),r(e,s)},p(i,l){var u;l&1&&c!==(c=((u=i[0])==null?void 0:u.include_special)+"")&&B(s,c),l&2&&I(e,"policyErr",!!i[1][5])},d(i){i&&p(e)}}}function se(n){var _;let e,a="Not one of last recent passwords: ",t,f,c=((_=n[0])==null?void 0:_.not_recently_used)+"",s;return{c(){e=L("li"),t=w(a),f=P(),s=w(c),this.h()},l(i){e=M(i,"LI",{class:!0});var l=C(e);t=b(l,a),f=q(l),s=b(l,c),l.forEach(p),this.h()},h(){V(e,"class","li svelte-qyge6w")},m(i,l){D(i,e,l),r(e,t),r(e,f),r(e,s)},p(i,l){var u;l&1&&c!==(c=((u=i[0])==null?void 0:u.not_recently_used)+"")&&B(s,c)},d(i){i&&p(e)}}}function fe(n){let e,a=n[0]&&x(n);return{c(){a&&a.c(),e=Z()},l(t){a&&a.l(t),e=Z()},m(t,f){a&&a.m(t,f),D(t,e,f)},p(t,[f]){t[0]?a?a.p(t,f):(a=x(t),a.c(),a.m(e.parentNode,e)):a&&(a.d(1),a=null)},i:$,o:$,d(t){a&&a.d(t),t&&p(e)}}}function _e(n,e,a){let{policy:t={}}=e,{password:f=""}=e,{accepted:c=!1}=e,s=[!1,!1,!1,!1,!1,!1];const _=i=>{if(!t)return!1;let l=[!1,!1,!1,!1,!1,!1],u=!1;i.length<t.length_min&&(l[0]=!0,u=!0),i.length>t.length_max&&(l[1]=!0,u=!0);let d=[0,0,0,0];for(let A=0;A<i.length;A++){let N=i.charCodeAt(A);if(N>=97&&N<=122){d[0]=d[0]+1;continue}if(N>=65&&N<=90){d[1]=d[1]+1;continue}if(N>=48&&N<=57){d[2]=d[2]+1;continue}d[3]=d[3]+1}t.include_lower_case!==-1&&t.include_lower_case>d[0]&&(l[2]=!0,u=!0),t.include_upper_case!==-1&&t.include_upper_case>d[1]&&(l[3]=!0,u=!0),t.include_digits!==-1&&t.include_digits>d[2]&&(l[4]=!0,u=!0),t.include_special!==-1&&t.include_special>d[3]&&(l[5]=!0,u=!0),a(1,s=l),a(2,c=!u)};return n.$$set=i=>{"policy"in i&&a(0,t=i.policy),"password"in i&&a(3,f=i.password),"accepted"in i&&a(2,c=i.accepted)},n.$$.update=()=>{n.$$.dirty&8&&f&&_(f)},[t,s,c,f]}class oe extends ae{constructor(e){super(),ce(this,e,_e,fe,ne,{policy:0,password:3,accepted:2})}}export{oe as P};
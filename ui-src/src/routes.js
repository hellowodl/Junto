import Pack from "./components/Pack/Pack.vue";
import Collective from "./components/Collective/Collective.vue";
import LotusOpen from "./components/Lotus/LotusOpen/LotusOpen.vue";
import User from "./components/User/User";
import Register from "./components/User/Register/Register";
import Den from "./components/User/Den/Den.vue";
import PublicDen from "./components/User/Den/Public/PublicDen";
import PrivateDen from "./components/User/Den/Private/PrivateDen";
import NotFound from "./components/NotFound/404.vue";

import { makeHolochainCall, isSuccess } from "./utils.js";
import { connect } from "@holochain/hc-web-client";
import { Settings } from "./settings.js";
import store from "./store.js";

const checkSourceChain = async (to, from, next) => {
  //check if user has account in their source chain and redirect accordingly
  let connection = connect({ url: Settings.Uri });
  if (to.path != "/user/register") {
    makeHolochainCall(
      connection,
      "user",
      "get_user_profile_by_agent_address",
      {},
      result => {
        if (isSuccess(result) == true) {
          console.log(
            "(checkSourceChain) User is already registered here is their metadata: ",
            result
          );
          next(next);
        } else {
          next("/register");
        }
      }
    );
  } else {
    next();
  }
};

export const routes = [
  { path: "/", component: Collective, beforeEnter: checkSourceChain },
  {
    path: "/user/:address",
    component: User,
    beforeEnter: checkSourceChain,
    props: true,
    children: [
      {
        path: "den",
        component: Den,
        children: [
          {
            path: "public",
            component: PublicDen
          },
          {
            path: "private",
            component: PrivateDen
          }
        ]
      }
    ]
  },
  {
    path: "/register",
    component: Register
  },
  { path: "/pack", component: Pack, beforeEnter: checkSourceChain },
  { path: "/collective", component: Collective, beforeEnter: checkSourceChain },
  { path: "/lotus", component: LotusOpen, beforeEnter: checkSourceChain },
  { path: "*", component: NotFound }
];

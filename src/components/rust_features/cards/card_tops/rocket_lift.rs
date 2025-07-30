use leptos::prelude::*;
use stylance::*;

use crate::cn;

import_style!(classes, "rocket_lift.module.scss");

#[component]
pub fn RocketLift() -> impl IntoView {
    view! {
        <div class="w-full h-full flex items-end justify-center bg-blue-200">


         <div class=classes::rocketWrapper>
          <div class=cn!("w-[120px] h-fit", classes::rocket)>

          <div class=classes::rocketBody>
            <div class=classes::rocketTop>
              <div class=classes::front />
              <div class=classes::right>
                // <div class=classes::inner />
              </div>
              <div class=classes::left />
              <div class=classes::back />
            </div>
          </div>

          </div>
         </div>

        </div>
    }
}

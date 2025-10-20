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
              <div class=classes::front>
                <div class=classes::face>
                 <div class=classes::innerLayer1 />
                 <div class=classes::innerLayer2 />
                  <div class=classes::strip>
                    <div class=classes::innerLayer1 />
                    <div class=classes::innerLayer2 />
                  </div>
               </div>
              </div>
              <div class=classes::right>
               <div class=classes::face>
                 <div class=classes::innerLayer1 />
                 <div class=classes::innerLayer2 />
                  <div class=classes::strip>
                    <div class=classes::innerLayer1 />
                    <div class=classes::innerLayer2 />
                  </div>
               </div>
              </div>
              <div class=classes::left>
                 <div class=classes::face>
                 <div class=classes::innerLayer1 />
                 <div class=classes::innerLayer2 />
                  <div class=classes::strip>
                    <div class=classes::innerLayer1 />
                    <div class=classes::innerLayer2 />
                  </div>
               </div>
              </div>
              <div class=classes::back>
                <div class=classes::face>
                  <div class=classes::innerLayer1 />
                  <div class=classes::innerLayer2 />
                  <div class=classes::strip>
                    <div class=classes::innerLayer1 />
                    <div class=classes::innerLayer2 />
                  </div>
               </div>
              </div>
            </div>

            <div class=classes::mainBody>
                <div class=cn!(classes::leftWing, classes::wing)>
                    <div class=classes::upper>
                        <div class=classes::front>
                            <div class=classes::face>
                                <div class=classes::innerLayer1 />
                                <div class=classes::innerLayer2 />
                            </div>
                        </div>
                        <div class=classes::back>
                            <div class=classes::face>
                                <div class=classes::innerLayer1 />
                                <div class=classes::innerLayer2 />
                            </div>
                        </div>
                        <div class=classes::top>
                            <div class=classes::face>
                                <div class=classes::innerLayer1 />
                                <div class=classes::innerLayer2 />
                            </div>
                        </div>
                        <div class=classes::bottom>
                            <div class=classes::face>
                                <div class=classes::innerLayer1 />
                                <div class=classes::innerLayer2 />
                            </div>
                        </div>
                    </div>

                    <div class=classes::lower>
                        <div class=classes::front>
                            <div class=classes::face>
                                <div class=classes::innerLayer1 />
                                <div class=classes::innerLayer2 />
                            </div>
                        </div>
                        <div class=classes::back>
                            <div class=classes::face>
                                <div class=classes::innerLayer1 />
                                <div class=classes::innerLayer2 />
                            </div>
                        </div>
                        <div class=classes::left>
                            <div class=classes::face>
                                <div class=classes::innerLayer1 />
                                <div class=classes::innerLayer2 />
                            </div>
                        </div>
                        <div class=classes::right>
                            <div class=classes::face>
                                <div class=classes::innerLayer1 />
                                <div class=classes::innerLayer2 />
                            </div>
                        </div>
                        // <div class=classes::top>
                        //     <div class=classes::face>
                        //         <div class=classes::innerLayer1 />
                        //         <div class=classes::innerLayer2 />
                        //     </div>
                        // </div>
                        // <div class=classes::bottom>
                        //     <div class=classes::face>
                        //         <div class=classes::innerLayer1 />
                        //         <div class=classes::innerLayer2 />
                        //     </div>
                        // </div>
                    </div>
                </div>

                <div class=cn!(classes::rightWing, classes::wing)>
                    <div class=classes::upper>
                        <div class=classes::front>
                            <div class=classes::face>
                                <div class=classes::innerLayer1 />
                                <div class=classes::innerLayer2 />
                            </div>
                        </div>
                        <div class=classes::back>
                            <div class=classes::face>
                                <div class=classes::innerLayer1 />
                                <div class=classes::innerLayer2 />
                            </div>
                        </div>
                        <div class=classes::top>
                            <div class=classes::face>
                                <div class=classes::innerLayer1 />
                                <div class=classes::innerLayer2 />
                            </div>
                        </div>
                        <div class=classes::bottom>
                            <div class=classes::face>
                                <div class=classes::innerLayer1 />
                                <div class=classes::innerLayer2 />
                            </div>
                        </div>
                    </div>
                </div>

                <div class=classes::front>
                    <div class=classes::face>
                        <div class=classes::innerLayer1 />
                        <div class=classes::innerLayer2 />
                    </div>
                </div>

                <div class=classes::back>
                    <div class=classes::face>
                    <div class=classes::innerLayer1 />
                    <div class=classes::innerLayer2 />
                    </div>
                </div>

                <div class=classes::left>
                    <div class=classes::face>
                        <div class=classes::innerLayer1 />
                        <div class=classes::innerLayer2 />
                    </div>
                </div>

                <div class=classes::right>
                    <div class=classes::face>
                        <div class=classes::innerLayer1 />
                        <div class=classes::innerLayer2 />
                    </div>
                </div>
            </div>
          </div>

          </div>
         </div>

        </div>
    }
}

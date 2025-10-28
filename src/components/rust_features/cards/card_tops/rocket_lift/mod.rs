use leptos::prelude::*;
use stylance::*;
use tailwind_fuse::tw_join;

import_style!(classes, "rocket_lift.module.scss");

#[component]
pub fn RocketLift() -> impl IntoView {
    view! {
        <div class="w-full h-full flex items-end justify-center bg-blue-200">
         <div class=tw_join!("rocketWrapper", classes::rocket)>
          <div class="rocketInner">
          <div class="rocketParts">
            <div class="rocketTop">
              <div class="front">
                <div class="face">
                 <div class="innerLayer1" />
                 <div class="innerLayer2" />
                  <div class="strip">
                    <div class="innerLayer1" />
                    <div class="innerLayer2" />
                  </div>
               </div>
              </div>
              <div class="right">
               <div class="face">
                 <div class="innerLayer1" />
                 <div class="innerLayer2" />
                  <div class="strip">
                    <div class="innerLayer1" />
                    <div class="innerLayer2" />
                  </div>
               </div>
              </div>
              <div class="left">
                 <div class="face">
                 <div class="innerLayer1" />
                 <div class="innerLayer2" />
                  <div class="strip">
                    <div class="innerLayer1" />
                    <div class="innerLayer2" />
                  </div>
               </div>
              </div>
              <div class="back">
                <div class="face">
                  <div class="innerLayer1" />
                  <div class="innerLayer2" />
                  <div class="strip">
                    <div class="innerLayer1" />
                    <div class="innerLayer2" />
                  </div>
               </div>
              </div>
            </div>

            <div class="mainBody">
                <div class="leftWing wing">
                    <div class="upper">
                        <div class="front">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="back">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="top">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="bottom">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="left">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                    </div>

                    <div class="lower">
                        <div class="front">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="back">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="left">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="right">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                    </div>
                </div>

                <div class="rightWing wing">
                    <div class="upper">
                        <div class="front">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="back">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="top">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="bottom">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="right">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                    </div>

                    <div class="lower">
                        <div class="front">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="back">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="left">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                        <div class="right">
                            <div class="face">
                                <div class="innerLayer1" />
                                <div class="innerLayer2" />
                            </div>
                        </div>
                    </div>
                </div>

                <div class="front">
                    <div class="window">
                        <div></div>
                    </div>
                    <div class="face">
                        <div class="innerLayer1" />
                        <div class="innerLayer2" />
                    </div>
                </div>

                <div class="back">
                    <div class="window">
                        <div></div>
                    </div>
                    <div class="face">
                        <div class="innerLayer1" />
                        <div class="innerLayer2" />
                    </div>
                </div>

                <div class="left">
                    <div class="face">
                        <div class="innerLayer1" />
                        <div class="innerLayer2" />
                    </div>
                </div>

                <div class="right">
                    <div class="face">
                        <div class="innerLayer1" />
                        <div class="innerLayer2" />
                    </div>
                </div>
            </div>
            <div class="emitter">
                <div class="front">
                    <div class="face">
                        <div />
                        <div />
                        <div />
                    </div>
                </div>

                <div class="back">
                    <div class="face">
                        <div />
                        <div />
                        <div />
                    </div>
                </div>

                <div class="left">
                    <div class="face">
                        <div />
                        <div />
                        <div />
                    </div>
                </div>

                <div class="right">
                    <div class="face">
                        <div />
                        <div />
                        <div />
                    </div>
                </div>
            </div>
          </div>
          </div>
         </div>
        </div>
    }
}

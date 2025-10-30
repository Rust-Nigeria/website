use leptos::prelude::*;
use stylance::*;
use tailwind_fuse::tw_join;

import_style!(classes, "rocket.module.scss");

#[component]
pub fn Rocket(
    class: Signal<String>,
    #[prop(optional)] inner_class: Option<Signal<String>>,
) -> impl IntoView {
    let inner_class = inner_class.unwrap_or_else(|| "".into());
    view! {
         <div class=move || tw_join!("rocketWrapper", class.get())>
          <div class="rocketInner">
          <div class=move || tw_join!("rocketParts", inner_class.get())>
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
    }
}

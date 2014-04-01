/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::HTMLLinkElementBinding;
use dom::bindings::codegen::InheritTypes::{ElementCast, HTMLLinkElementDerived};
use dom::bindings::js::JS;
use dom::bindings::error::ErrorResult;
use dom::document::Document;
use dom::element::{AttributeHandlers, AfterSetAttrListener, BeforeRemoveAttrListener};
use dom::element::{Element, HTMLLinkElementTypeId};
use dom::eventtarget::{EventTarget, NodeTargetTypeId};
use dom::htmlelement::HTMLElement;
use dom::node::{Node, ElementNodeTypeId, window_from_node};
use servo_util::namespace::Null;
use servo_util::str::{DOMString, HTML_SPACE_CHARACTERS};
use servo_util::url::parse_url;

#[deriving(Encodable)]
pub struct HTMLLinkElement {
    htmlelement: HTMLElement,
}

impl HTMLLinkElementDerived for EventTarget {
    fn is_htmllinkelement(&self) -> bool {
        match self.type_id {
            NodeTargetTypeId(ElementNodeTypeId(HTMLLinkElementTypeId)) => true,
            _ => false
        }
    }
}

impl HTMLLinkElement {
    pub fn new_inherited(localName: DOMString, document: JS<Document>) -> HTMLLinkElement {
        HTMLLinkElement {
            htmlelement: HTMLElement::new_inherited(HTMLLinkElementTypeId, localName, document)
        }
    }

    pub fn new(localName: DOMString, document: &JS<Document>) -> JS<HTMLLinkElement> {
        let element = HTMLLinkElement::new_inherited(localName, document.clone());
        Node::reflect_node(~element, document, HTMLLinkElementBinding::Wrap)
    }
}

impl HTMLLinkElement {
    pub fn Disabled(&self) -> bool {
        false
    }

    pub fn SetDisabled(&mut self, _disable: bool) {
    }

    pub fn Href(&self) -> DOMString {
        ~""
    }

    pub fn SetHref(&mut self, _href: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn CrossOrigin(&self) -> DOMString {
        ~""
    }

    pub fn SetCrossOrigin(&mut self, _cross_origin: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Rel(&self) -> DOMString {
        ~""
    }

    pub fn SetRel(&mut self, _rel: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Media(&self) -> DOMString {
        ~""
    }

    pub fn SetMedia(&mut self, _media: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Hreflang(&self) -> DOMString {
        ~""
    }

    pub fn SetHreflang(&mut self, _href: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Type(&self) -> DOMString {
        ~""
    }

    pub fn SetType(&mut self, _type: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Charset(&self) -> DOMString {
        ~""
    }

    pub fn SetCharset(&mut self, _charset: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Rev(&self) -> DOMString {
        ~""
    }

    pub fn SetRev(&mut self, _rev: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Target(&self) -> DOMString {
        ~""
    }

    pub fn SetTarget(&mut self, _target: DOMString) -> ErrorResult {
        Ok(())
    }
}

impl AfterSetAttrListener for JS<HTMLLinkElement> {
    fn AfterSetAttr(&mut self, name: DOMString, value: DOMString) {
        let elem: JS<Element> = ElementCast::from(self);
        if "rel" == name || "href" == name {
            match (elem.get_attribute(Null, "rel"), elem.get_attribute(Null, "href")) {
                (Some(ref rel), Some(ref href)) if rel.get()
                                                      .value_ref()
                                                      .split(HTML_SPACE_CHARACTERS.
                                                             as_slice())
                                                      .any(|s| {
                            s.eq_ignore_ascii_case("stylesheet")
                        }) => {
                    debug!("found CSS stylesheet: {:s}", href.get().value_ref());
                    let window = window_from_node(self);
                    let base_url = Some(window.get().get_url());
                    let url = parse_url(href.get().value_ref(), Some(base_url.clone()));
                    //css_chan2.send(CSSTaskNewFile(UrlProvenance(url)));
                }
                _ => ()
            }
        }
    }
}


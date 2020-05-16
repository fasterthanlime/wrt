#!/bin/sh -xe
wmdgen windows.foundation windows.foundation.collections windows.foundation.numerics windows.ui windows.ui.composition windows.ui.composition.desktop windows.graphics windows.system windows.web.syndication windows.applicationmodel.core --output ./src/generated.rs

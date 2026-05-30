(function () {
  var maps = typeof WeakMap !== "undefined" ? new WeakMap() : null;
  var fallbackStore = Object.create(null);

  function store(el, state) {
    if (maps) {
      maps.set(el, state);
    } else {
      fallbackStore[el.id || "default"] = state;
    }
  }

  function load(el) {
    if (maps) {
      return maps.get(el);
    }
    return fallbackStore[el.id || "default"];
  }

  function remove(el) {
    if (maps) {
      maps.delete(el);
    } else {
      delete fallbackStore[el.id || "default"];
    }
  }

  function pinRadius(count) {
    if (!count) {
      return 14;
    }
    return Math.min(28, Math.sqrt(count) * 5 + 16);
  }

  function markerHtml(label, count, active, hovered, live) {
    var classes = ["sm-osm-marker"];
    if (live) classes.push("sm-osm-marker-live");
    if (active) classes.push("sm-osm-marker-active");
    if (hovered) classes.push("sm-osm-marker-hover");

    var countHtml =
      count > 0
        ? '<span class="sm-osm-marker-count">' + count + "</span>"
        : "";

    return (
      '<div class="' +
      classes.join(" ") +
      '">' +
      countHtml +
      '<span class="sm-osm-marker-dot"></span>' +
      '<span class="sm-osm-marker-label">' +
      label +
      "</span>" +
      "</div>"
    );
  }

  function makeIcon(label, count, active, hovered) {
    var live = count > 0;
    var size = pinRadius(count) * 2 + 24;
    return L.divIcon({
      className: "sm-osm-marker-wrap",
      html: markerHtml(label, count, active, hovered, live),
      iconSize: [size, size],
      iconAnchor: [size / 2, size / 2],
    });
  }

  function applyMarkerVisual(marker, pin, count, active, hovered) {
    var meta = marker._smMeta || { visualKey: "", hovered: false };
    var visualKey = String(count) + "|" + (active ? "1" : "0");

    if (meta.visualKey !== visualKey) {
      marker.setIcon(makeIcon(pin.label, count, active, false));
      meta = { visualKey: visualKey, hovered: false };
      marker._smMeta = meta;
    }

    if (meta.hovered !== hovered) {
      meta.hovered = hovered;
      marker._smMeta = meta;
      var el = marker.getElement();
      if (el) {
        var inner = el.querySelector(".sm-osm-marker");
        if (inner) {
          inner.classList.toggle("sm-osm-marker-hover", hovered);
        }
      }
    }
  }

  function scheduleHoverClear(state) {
    if (state.hoverClearTimer) {
      clearTimeout(state.hoverClearTimer);
    }
    state.hoverClearTimer = setTimeout(function () {
      state.hoverClearTimer = null;
      if (state.callbacks && state.callbacks.onHover) {
        state.callbacks.onHover(null);
      }
    }, 200);
  }

  function cancelHoverClear(state) {
    if (state.hoverClearTimer) {
      clearTimeout(state.hoverClearTimer);
      state.hoverClearTimer = null;
    }
  }

  function findPin(pins, regionId) {
    for (var i = 0; i < pins.length; i++) {
      if (pins[i].id === regionId) {
        return pins[i];
      }
    }
    return null;
  }

  window.SituationMap = {
    mount: function (el, pins, callbacks) {
      if (!window.L) {
        throw new Error("Leaflet is not loaded");
      }

      var map = L.map(el, {
        center: [18, 10],
        zoom: 2,
        minZoom: 2,
        maxZoom: 12,
        worldCopyJump: true,
        scrollWheelZoom: true,
        attributionControl: true,
      });

      L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
        maxZoom: 19,
        attribution:
          '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
      }).addTo(map);

      var markers = Object.create(null);
      pins.forEach(function (pin) {
        var marker = L.marker([pin.lat, pin.lon], {
          icon: makeIcon(pin.label, 0, false, false),
          riseOnHover: true,
        }).addTo(map);

        marker.on("click", function () {
          if (callbacks && callbacks.onSelect) {
            callbacks.onSelect(pin.id);
          }
        });
        marker.on("mouseover", function () {
          var state = load(el);
          if (state) {
            cancelHoverClear(state);
          }
          if (callbacks && callbacks.onHover) {
            callbacks.onHover(pin.id);
          }
        });
        marker.on("mouseout", function () {
          var state = load(el);
          if (state) {
            scheduleHoverClear(state);
          }
        });
        marker.on("dblclick", function (event) {
          if (event && event.originalEvent) {
            L.DomEvent.stopPropagation(event.originalEvent);
          }
          map.flyTo([pin.lat, pin.lon], Math.max(map.getZoom(), 4));
          if (callbacks && callbacks.onSelect) {
            callbacks.onSelect(pin.id);
          }
        });

        markers[pin.id] = marker;
      });

      var state = {
        map: map,
        markers: markers,
        pins: pins,
        hovered: null,
        active: null,
        counts: Object.create(null),
        callbacks: callbacks,
        hoverClearTimer: null,
      };
      store(el, state);

      setTimeout(function () {
        map.invalidateSize();
      }, 0);

      return true;
    },

    update: function (el, counts, activeRegion) {
      var state = load(el);
      if (!state) {
        return;
      }

      state.counts = counts || Object.create(null);
      state.active = activeRegion || null;

      state.pins.forEach(function (pin) {
        var marker = state.markers[pin.id];
        if (!marker) {
          return;
        }
        var count = state.counts[pin.id] || 0;
        var active = state.active === pin.id;
        var hovered = state.hovered === pin.id;
        applyMarkerVisual(marker, pin, count, active, hovered);
      });
    },

    setHover: function (el, hoveredRegion) {
      var state = load(el);
      if (!state) {
        return;
      }

      cancelHoverClear(state);
      state.hovered = hoveredRegion || null;

      state.pins.forEach(function (pin) {
        var marker = state.markers[pin.id];
        if (!marker) {
          return;
        }
        var count = state.counts[pin.id] || 0;
        var active = state.active === pin.id;
        var hovered = state.hovered === pin.id;
        applyMarkerVisual(marker, pin, count, active, hovered);
      });
    },

    flyTo: function (el, regionId) {
      var state = load(el);
      if (!state) {
        return;
      }
      var pin = findPin(state.pins, regionId);
      if (!pin) {
        return;
      }
      state.map.flyTo([pin.lat, pin.lon], Math.max(state.map.getZoom(), 4), {
        duration: 0.8,
      });
    },

    resetView: function (el) {
      var state = load(el);
      if (!state) {
        return;
      }
      state.map.setView([18, 10], 2, { animate: true });
    },

    zoomIn: function (el) {
      var state = load(el);
      if (state) {
        state.map.zoomIn();
      }
    },

    zoomOut: function (el) {
      var state = load(el);
      if (state) {
        state.map.zoomOut();
      }
    },

    unmount: function (el) {
      var state = load(el);
      if (!state) {
        return;
      }
      state.map.remove();
      remove(el);
    },
  };
})();

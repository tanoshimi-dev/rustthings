use std::f64::consts::TAU;

use wasm_bindgen::{JsValue, prelude::*};

#[wasm_bindgen]
pub fn vertex_shader_source() -> String {
    VERTEX_SHADER.to_string()
}

#[wasm_bindgen]
pub fn fragment_shader_source() -> String {
    FRAGMENT_SHADER.to_string()
}

#[wasm_bindgen]
pub fn scene_state(time: f64, theme: &str, intensity: f64) -> Result<String, JsValue> {
    scene_state_text(time, theme, intensity).map_err(|error| JsValue::from_str(&error))
}

#[wasm_bindgen]
pub fn describe_theme(theme: &str) -> String {
    SceneTheme::from_name(theme)
        .map(|theme| theme.description.to_string())
        .unwrap_or_else(|| "Unknown theme. Try synthwave, ember, or glacier.".to_string())
}

fn scene_state_text(time: f64, theme: &str, intensity: f64) -> Result<String, String> {
    if !(0.0..=2.0).contains(&intensity) {
        return Err("intensity must be between 0.0 and 2.0".to_string());
    }

    let theme = SceneTheme::from_name(theme)
        .ok_or_else(|| "theme must be one of: synthwave, ember, glacier".to_string())?;

    let orbit_speed = 0.18 + intensity * 0.12;
    let orbit = time * orbit_speed + theme.phase_offset;
    let camera_radius = 4.2 + intensity * 0.25;
    let camera_x = orbit.cos() * camera_radius;
    let camera_z = orbit.sin() * camera_radius;
    let camera_y = 1.15 + (time * 0.42).sin() * 0.18 * intensity;
    let glow = 0.55 + 0.25 * (((time * 1.1) + theme.phase_offset).sin() * 0.5 + 0.5);
    let pulse = 0.72 + 0.28 * (((time * 1.7) + theme.phase_offset * 0.4).sin() * 0.5 + 0.5);
    let spin = time * (0.32 + intensity * 0.22);
    let tilt = theme.ring_tilt + (time * 0.23).sin() * 0.12 * intensity;
    let grid = 0.9 + intensity * 0.35;

    Ok(format!(
        "cam={},{},{};accent={},{},{};shadow={},{},{};fog={},{},{};glow={};pulse={};spin={};tilt={};grid={}",
        trim_float(camera_x),
        trim_float(camera_y),
        trim_float(camera_z),
        trim_float(theme.accent[0]),
        trim_float(theme.accent[1]),
        trim_float(theme.accent[2]),
        trim_float(theme.shadow[0]),
        trim_float(theme.shadow[1]),
        trim_float(theme.shadow[2]),
        trim_float(theme.fog[0]),
        trim_float(theme.fog[1]),
        trim_float(theme.fog[2]),
        trim_float(glow),
        trim_float(pulse),
        trim_float(spin),
        trim_float(tilt),
        trim_float(grid),
    ))
}

fn trim_float(value: f64) -> String {
    let formatted = format!("{value:.4}");
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

struct SceneTheme {
    accent: [f64; 3],
    shadow: [f64; 3],
    fog: [f64; 3],
    ring_tilt: f64,
    phase_offset: f64,
    description: &'static str,
}

impl SceneTheme {
    fn from_name(name: &str) -> Option<&'static Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "synthwave" => Some(&SYNTHWAVE),
            "ember" => Some(&EMBER),
            "glacier" => Some(&GLACIER),
            _ => None,
        }
    }
}

const SYNTHWAVE: SceneTheme = SceneTheme {
    accent: [1.0, 0.36, 0.88],
    shadow: [0.06, 0.02, 0.13],
    fog: [0.17, 0.47, 0.96],
    ring_tilt: 0.8,
    phase_offset: 0.0,
    description: "Synthwave uses magenta light, deep purple shadows, and cool blue fog.",
};

const EMBER: SceneTheme = SceneTheme {
    accent: [1.0, 0.53, 0.22],
    shadow: [0.12, 0.04, 0.01],
    fog: [0.65, 0.2, 0.11],
    ring_tilt: 0.56,
    phase_offset: TAU / 5.0,
    description: "Ember pushes the scene warmer with orange glow, darker smoke, and fiery haze.",
};

const GLACIER: SceneTheme = SceneTheme {
    accent: [0.52, 0.95, 1.0],
    shadow: [0.01, 0.06, 0.12],
    fog: [0.38, 0.71, 0.94],
    ring_tilt: 1.05,
    phase_offset: TAU / 3.0,
    description: "Glacier cools everything down with cyan highlights and icy atmospheric fog.",
};

const VERTEX_SHADER: &str = r#"#version 300 es
in vec2 a_position;

void main() {
  gl_Position = vec4(a_position, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

uniform vec2 u_resolution;
uniform float u_time;
uniform vec3 u_cam;
uniform vec3 u_accent;
uniform vec3 u_shadow;
uniform vec3 u_fog;
uniform float u_glow;
uniform float u_pulse;
uniform float u_spin;
uniform float u_tilt;
uniform float u_grid;

out vec4 outColor;

mat2 rot(float angle) {
  float c = cos(angle);
  float s = sin(angle);
  return mat2(c, -s, s, c);
}

float sdSphere(vec3 p, float radius) {
  return length(p) - radius;
}

float sdPlane(vec3 p) {
  return p.y + 1.1;
}

float sdTorus(vec3 p, vec2 size) {
  vec2 q = vec2(length(p.xz) - size.x, p.y);
  return length(q) - size.y;
}

float sdBox(vec3 p, vec3 bounds) {
  vec3 q = abs(p) - bounds;
  return length(max(q, 0.0)) + min(max(q.x, max(q.y, q.z)), 0.0);
}

vec2 mapScene(vec3 p) {
  float distance = sdPlane(p);
  float material = 1.0;

  vec3 orb = p - vec3(0.0, 0.14 + 0.08 * sin(u_time * 1.1), 0.0);
  orb.xz *= rot(u_spin);
  float planet = sdSphere(orb, 0.95 + 0.06 * u_pulse);
  if (planet < distance) {
    distance = planet;
    material = 2.0;
  }

  vec3 ring = p - vec3(0.0, 0.08, 0.0);
  ring.yz *= rot(u_tilt);
  ring.xz *= rot(-u_spin * 0.75);
  float band = sdTorus(ring, vec2(1.45, 0.09));
  if (band < distance) {
    distance = band;
    material = 3.0;
  }

  vec3 pillars = p - vec3(0.0, -0.15, 0.0);
  pillars.z = mod(pillars.z + 2.5, 5.0) - 2.5;
  pillars.x = abs(pillars.x) - 2.3;
  float columns = sdBox(pillars, vec3(0.18, 0.9, 0.18));
  if (columns < distance) {
    distance = columns;
    material = 4.0;
  }

  return vec2(distance, material);
}

vec3 estimateNormal(vec3 p) {
  vec2 e = vec2(0.001, 0.0);
  float d = mapScene(p).x;
  return normalize(vec3(
    mapScene(p + e.xyy).x - d,
    mapScene(p + e.yxy).x - d,
    mapScene(p + e.yyx).x - d
  ));
}

float raymarch(vec3 ro, vec3 rd, out float material) {
  float travel = 0.0;
  material = -1.0;

  for (int step = 0; step < 100; step++) {
    vec3 p = ro + rd * travel;
    vec2 hit = mapScene(p);
    if (hit.x < 0.001) {
      material = hit.y;
      return travel;
    }

    if (travel > 24.0) {
      break;
    }

    travel += hit.x * 0.75;
  }

  return -1.0;
}

mat3 camera(vec3 ro, vec3 ta) {
  vec3 ww = normalize(ta - ro);
  vec3 uu = normalize(cross(ww, vec3(0.0, 1.0, 0.0)));
  vec3 vv = cross(uu, ww);
  return mat3(uu, vv, ww);
}

vec3 background(vec3 rd) {
  vec3 base = mix(u_fog, u_shadow, smoothstep(-0.45, 0.9, rd.y));
  float horizonGlow = exp(-10.0 * abs(rd.y + 0.1));
  base += u_accent * horizonGlow * 0.22 * u_glow;
  base += vec3(0.02, 0.02, 0.03) * pow(max(0.0, 1.0 - rd.y), 2.0);
  return base;
}

float floorGrid(vec3 p) {
  vec2 grid = abs(fract(p.xz * u_grid) - 0.5);
  float line = min(grid.x, grid.y);
  return 1.0 - smoothstep(0.0, 0.045, line);
}

vec3 shade(vec3 ro, vec3 rd, float travel, float material) {
  vec3 p = ro + rd * travel;
  vec3 normal = estimateNormal(p);
  vec3 lightDir = normalize(vec3(0.7, 1.2, -0.6));

  float diff = max(dot(normal, lightDir), 0.0);
  float rim = pow(1.0 - max(dot(normal, -rd), 0.0), 3.0);
  float spec = pow(max(dot(reflect(-lightDir, normal), -rd), 0.0), 32.0);

  vec3 base = u_shadow;

  if (material < 1.5) {
    float grid = floorGrid(p);
    base = mix(u_shadow * 0.72, u_accent, grid * 0.9);
  } else if (material < 2.5) {
    float bands = 0.5 + 0.5 * sin((p.y + u_spin * 0.2) * 10.0);
    base = mix(u_shadow, u_accent * 1.05, bands * 0.75 + 0.2);
  } else if (material < 3.5) {
    base = mix(vec3(0.95), u_accent, 0.6);
  } else {
    base = mix(u_shadow, u_fog + u_accent * 0.45, 0.55);
  }

  vec3 color = base * (0.18 + 0.82 * diff);
  color += u_accent * rim * (0.5 + 0.45 * u_glow);
  color += vec3(1.0) * spec * 0.25;

  float fog = 1.0 - exp(-0.025 * travel * travel);
  return mix(color, background(rd), fog);
}

void main() {
  vec2 uv = (gl_FragCoord.xy * 2.0 - u_resolution) / min(u_resolution.x, u_resolution.y);

  vec3 ro = u_cam;
  vec3 ta = vec3(0.0, 0.0, 0.0);
  vec3 rd = normalize(camera(ro, ta) * vec3(uv, 1.7));

  float material;
  float travel = raymarch(ro, rd, material);

  vec3 color = background(rd);
  if (travel > 0.0) {
    color = shade(ro, rd, travel, material);
  }

  float vignette = smoothstep(1.55, 0.15, length(uv));
  color *= vignette;
  color = pow(color, vec3(0.92));

  outColor = vec4(color, 1.0);
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_payload_contains_expected_keys() {
        let payload = scene_state_text(1.5, "synthwave", 1.0).unwrap();
        assert!(payload.contains("cam="));
        assert!(payload.contains("accent="));
        assert!(payload.contains("grid="));
    }

    #[test]
    fn invalid_theme_returns_error() {
        let error = scene_state_text(0.0, "forest", 1.0).unwrap_err();
        assert_eq!(error, "theme must be one of: synthwave, ember, glacier");
    }

    #[test]
    fn shader_sources_include_expected_markers() {
        assert!(vertex_shader_source().contains("#version 300 es"));
        assert!(fragment_shader_source().contains("u_resolution"));
        assert!(fragment_shader_source().contains("mapScene"));
    }
}
